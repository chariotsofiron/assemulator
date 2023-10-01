mod expression;
mod macros;

use crate::{
    cpu::{Cpu, Token},
    port::Port,
    util::mask,
};
use macros::Macro;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

/// Pest parser for the assembler.
#[derive(pest_derive::Parser)]
#[grammar = "./src/assembler/grammar.pest"]
struct AsmParser;

/// Pretty prints error.
fn format_error(pair: &Pair<Rule>, msg: &str) -> String {
    pest::error::Error::<Rule>::new_from_span(
        pest::error::ErrorVariant::CustomError {
            message: msg.to_owned(),
        },
        pair.as_span(),
    )
    .to_string()
}

fn interpret_escaped_chars(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('\\') => out.push('\\'),
                Some('\'') => out.push('\''),
                Some('\"') => out.push('\"'),
                Some('0') => out.push('\0'),
                Some(x) => out.push(x),
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub struct Assembler<T> {
    /// The path to the root assembly file.
    path: PathBuf,
    current_label: Option<String>,
    pub program: Vec<u8>,
    pub symbols: HashMap<String, u64>,
    pub data: Vec<u8>,
    macros: HashMap<String, Macro>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Cpu> Assembler<T> {
    pub fn assemble(path: &Path) -> Result<Assembler<T>, String> {
        let symbols: HashMap<_, _> = Port::iter()
            .map(|x| (format!("{x:?}").to_ascii_lowercase(), usize::from(x) as u64))
            .collect();

        let mut asm = Self {
            path: path.to_owned(),
            current_label: None,
            program: Vec::new(),
            symbols,
            data: Vec::new(),
            macros: HashMap::new(),
            phantom: std::marker::PhantomData,
        };
        asm.include(path, false)?;
        asm.data.clear();
        asm.program.clear();
        asm.include(path, true)?;
        Ok(asm)
    }

    fn parse_expression<U>(&self, arg: &Pair<Rule>, second_pass: bool) -> Result<U, String>
    where
        U: TryFrom<u64> + Copy,
    {
        let value: u64 = match arg.as_rule() {
            Rule::expression => expression::parse(arg.as_str(), &self.symbols, second_pass)
                .map_err(|e| format_error(arg, &format!("Failed to parse expression: {}", e))),
            Rule::character => {
                let text = interpret_escaped_chars(arg.clone().into_inner().as_str());
                Ok(u64::from(text.chars().next().unwrap()))
            }
            _ => Err(format_error(&arg, "Unexpected token")),
        }?;

        let n_bits = 8 * std::mem::size_of::<U>();
        mask(value, n_bits).map_err(|err| format_error(&arg, &err))
    }

    /// Takes lines for multi-line directives (macro, if)
    fn handle_directive(
        &mut self,
        directive: Pair<Rule>,
        second_pass: bool,
        lines: &mut Pairs<Rule>,
    ) -> Result<(), String> {
        let mut tokens = directive.into_inner();
        let command = tokens.next().unwrap();

        match command.as_str() {
            "macro" => {
                let body: String = lines
                    .take_while(|x| x.as_str().trim() != ".endm")
                    .map(|x| format!("{}\n", x.as_str()))
                    .collect();

                if !second_pass {
                    let args = tokens.map(|x| x.as_str().to_owned()).collect::<Vec<_>>();
                    let mac = Macro::new(args, body);
                    self.declare_macro(mac)?;
                }
            }
            "if" => {
                let expr = tokens
                    .next()
                    .ok_or(format_error(&command, "Expected at least one argument"))?;

                let value = self.parse_expression::<u64>(&expr, true)?;
                if value == 0 {
                    for line in lines {
                        if line.as_str().trim() == ".endif" {
                            break;
                        }
                    }
                }
            }
            "endif" => {}
            "include" => {
                let filename = tokens.next().expect("Filename").into_inner().as_str();
                let filename = self.path.parent().unwrap().join(filename);
                self.include(&filename, second_pass)?;
            }
            "set" => {
                if !second_pass {
                    let expr = tokens
                        .next()
                        .ok_or(format_error(&command, "Expected at least one argument"))?;
                    self.declare_label(self.parse_expression(&expr, true)?)?;
                }
            }
            "i8" => {
                if !second_pass {
                    self.declare_label(self.data.len() as u64)?;
                }
                for arg in tokens {
                    let value = self.parse_expression::<u8>(&arg, second_pass)?;
                    self.data.push(value);
                }
            }
            "i16" => {
                if !second_pass {
                    self.declare_label(self.data.len() as u64)?;
                }
                for arg in tokens {
                    let value = self.parse_expression::<u16>(&arg, second_pass)?;
                    self.data.extend(value.to_be_bytes());
                }
            }
            "zero" => {
                if !second_pass {
                    self.declare_label(self.data.len() as u64)?;
                }
                let count = tokens
                    .next()
                    .ok_or(format_error(&command, "Expected at least one argument"))?;
                let count = self.parse_expression::<u64>(&count, second_pass)?;
                for _ in 0..count {
                    self.data.push(0);
                }
            }
            "strz" => {
                if !second_pass {
                    self.declare_label(self.data.len() as u64)?;
                }
                let string = interpret_escaped_chars(
                    tokens
                        .next()
                        .ok_or(format_error(&command, "Missing string"))?
                        .as_str(),
                );
                self.data.extend(string.as_bytes());
                self.data.push(0);
            }
            x => return Err(format_error(&command, &format!("unknown directive: {x}"))),
        }
        Ok(())
    }

    /// Declares the current label with the given value.
    fn declare_label(&mut self, value: u64) -> Result<(), String> {
        if let Some(label) = self.current_label.take() {
            if self.macros.contains_key(&label) || self.symbols.contains_key(&label) {
                return Err(format!("label already defined: {label}"));
            }
            self.symbols.insert(label, value);
        }
        Ok(())
    }

    fn declare_macro(&mut self, value: Macro) -> Result<(), String> {
        if let Some(label) = self.current_label.take() {
            if self.macros.contains_key(&label) || self.symbols.contains_key(&label) {
                return Err("macro already defined".to_owned());
            }
            self.macros.insert(label, value);
        }
        Ok(())
    }

    /// Expands a macro.
    fn expand_macro(&mut self, args: Pair<Rule>, second_pass: bool) -> Result<(), String> {
        let mut tokens = args.into_inner().map(|x| x.as_str());
        let opcode = tokens.next().unwrap();
        let args = tokens.collect::<Vec<_>>();
        let body = self.macros[opcode].expand(&args);

        let mut lines = AsmParser::parse(Rule::lines, &body).map_err(|e| e.to_string())?;
        self.parse_line(second_pass, &mut lines)?;
        Ok(())
    }

    fn decode_instruction(&self, arg: Pair<Rule>, second_pass: bool) -> Result<Vec<u8>, String> {
        let tokens = arg
            .clone()
            .into_inner()
            .map(|x| {
                if let Ok(opcode) = T::Opcode::try_from(x.as_str()) {
                    Ok(Token::Op(opcode))
                } else if let Ok(reg) = T::Reg::try_from(x.as_str()) {
                    Ok(Token::Reg(reg))
                } else {
                    self.parse_expression(&x, second_pass).map(Token::Imm)
                }
            })
            .collect::<Result<Vec<_>, String>>()?;
        let bytes = T::parse(tokens, self.program.len()).map_err(|err| format_error(&arg, &err))?;
        Ok(bytes)
    }

    fn parse_line(&mut self, second_pass: bool, lines: &mut Pairs<Rule>) -> Result<(), String> {
        while let Some(pair) = lines.next() {
            match pair.as_rule() {
                Rule::label => self.current_label = Some(pair.as_str().to_owned()),
                Rule::directive => {
                    self.handle_directive(pair, second_pass, lines)?;
                    self.current_label = None;
                }
                Rule::inst => {
                    let opcode = pair
                        .clone()
                        .into_inner()
                        .next()
                        .expect("Expected opcode")
                        .as_str();
                    if self.macros.contains_key(opcode) {
                        self.expand_macro(pair, second_pass)?;
                    } else {
                        if !second_pass {
                            self.declare_label(self.program.len() as u64)?;
                        }
                        self.program
                            .extend(self.decode_instruction(pair, second_pass)?);
                    }
                    self.current_label = None;
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    /// Includes a file into the current assembly.
    ///
    /// This is expected to be called twice, once for the first pass and again
    /// for the second pass. The first pass will declare all labels and macros.
    /// The second pass will generate the assembly.
    fn include(&mut self, filename: &Path, second_pass: bool) -> Result<(), String> {
        let mut text = std::fs::read_to_string(filename)
            .map_err(|_| format!("Failed to read {}", filename.display()))?;
        text.push('\n'); // add newline to fix pest grammar issue

        let mut lines = AsmParser::parse(Rule::lines, &text).map_err(|e| e.to_string())?;
        self.parse_line(second_pass, &mut lines)?;
        // if there's a dangling label, treat it as a code label
        if !second_pass {
            self.declare_label(self.program.len() as u64)?;
        }
        Ok(())
    }
}
