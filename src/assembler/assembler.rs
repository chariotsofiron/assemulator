use super::macros::Macro;
use crate::{
    cpu::{Cpu, Token},
    port::Port,
    util::{mask, parse_int},
};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use strum::IntoEnumIterator;

/// Pest parser for the assembler.
#[derive(Parser)]
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

    fn parse_expression<U>(&self, arg: Pair<Rule>, second_pass: bool) -> Result<U, String>
    where
        U: TryFrom<u64> + Copy,
    {
        let value: u64 = match arg.as_rule() {
            Rule::number => parse_int(arg.as_str()).map_err(|err| format_error(&arg, &err)),
            Rule::identifier => {
                if second_pass {
                    self.symbols
                        .get(arg.as_str())
                        .copied()
                        .ok_or_else(|| format_error(&arg, "Undefined label"))
                } else {
                    Ok(0)
                }
            }
            Rule::character => {
                let text = interpret_escaped_chars(arg.clone().into_inner().as_str());
                Ok(u64::from(text.chars().next().unwrap()))
            }
            _ => Err(format_error(&arg, "Unexpected token")),
        }?;

        let n_bits = 8 * std::mem::size_of::<U>();
        mask(value, n_bits).map_err(|err| format_error(&arg, &err))
    }

    fn handle_directive(&mut self, directive: Pair<Rule>, second_pass: bool) -> Result<(), String> {
        let mut tokens = directive.into_inner();
        let command = tokens.next().unwrap();

        match command.as_str() {
            "include" => {
                let filename = tokens.next().expect("Filename").into_inner().as_str();
                let filename = self.path.parent().unwrap().join(filename);
                self.include(&filename, second_pass)?;
            }
            "set" => {
                if second_pass {
                    let expr = tokens
                        .next()
                        .ok_or(format_error(&command, "Expected at least one argument"))?;
                    self.declare_label(self.parse_expression(expr, second_pass)?)?;
                }
            }
            "i8" => {
                if !second_pass {
                    self.declare_label(self.data.len() as u64)?;
                }
                for arg in tokens {
                    let value = self.parse_expression::<u8>(arg, second_pass)?;
                    self.data.push(value);
                }
            }
            "i16" => {
                if !second_pass {
                    self.declare_label(self.data.len() as u64)?;
                }
                for arg in tokens {
                    let value = self.parse_expression::<u16>(arg, second_pass)?;
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
                let count = self.parse_expression::<u64>(count, second_pass)?;
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
        if let Some(label) = std::mem::replace(&mut self.current_label, None) {
            if self.macros.contains_key(&label) || self.symbols.contains_key(&label) {
                return Err(format!("label already defined: {label}"));
            }
            self.symbols.insert(label, value);
        }
        Ok(())
    }

    fn declare_macro(&mut self, value: Macro) -> Result<(), String> {
        if let Some(label) = std::mem::replace(&mut self.current_label, None) {
            if self.macros.contains_key(&label) || self.symbols.contains_key(&label) {
                return Err(format!("macro already defined"));
            }
            self.macros.insert(label, value);
        }
        Ok(())
    }

    fn parse_macro<'a>(
        &mut self,
        pair: Pair<Rule>,
        pairs: &mut Pairs<Rule>,
    ) -> Result<Macro, String> {
        // need to handle macros here so we can manually advance lines
        // let mut instructions = Vec::new();

        let args = pair.into_inner().skip(1);
        let mut text = String::new();
        for pair in pairs {
            let line = pair.as_str();
            if line.trim() == ".endm" {
                break;
            }
            text.push_str(line);
            text.push('\n');
        }

        let args2: Vec<_> = args.map(|x| x.as_str().to_owned()).collect::<Vec<_>>();
        let mac = Macro::new(args2, text);
        Ok(mac)
    }

    /// Expands a macro.
    fn expand_macro(&mut self, args: Pair<Rule>, second_pass: bool) -> Result<(), String> {
        let mut tokens = args.into_inner().into_iter().map(|x| x.as_str());
        let opcode = tokens.next().expect("Token");
        let args = tokens.collect::<Vec<_>>();
        let body = self.macros[opcode].expand(&args);
        let lines = AsmParser::parse(Rule::lines, &body).map_err(|e| e.to_string())?;

        for line in lines {
            match line.as_rule() {
                Rule::label => self.current_label = Some(line.as_str().to_owned()),
                Rule::dir | Rule::inst => self.parse_line(line, second_pass)?,
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }
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
                    self.parse_expression(x, second_pass).map(Token::Imm)
                }
            })
            .collect::<Result<Vec<_>, String>>()?;
        let bytes = T::parse(tokens, self.program.len()).map_err(|err| format_error(&arg, &err))?;
        Ok(bytes)
    }

    fn parse_line(&mut self, pair: Pair<Rule>, second_pass: bool) -> Result<(), String> {
        match pair.as_rule() {
            Rule::dir => self.handle_directive(pair, second_pass)?,
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
            }
            _ => unreachable!(),
        }
        // the label may not have been reset depending on which pass we're on
        self.current_label = None;
        Ok(())
    }

    /// Includes a file into the current assembly.
    fn include(&mut self, filename: &Path, second_pass: bool) -> Result<(), String> {
        let mut text = std::fs::read_to_string(filename).expect("Failed to read file.");
        text.push('\n'); // add newline to fix pest grammar issue
        let mut lines = AsmParser::parse(Rule::lines, &text).map_err(|e| e.to_string())?;
        while let Some(pair) = lines.next() {
            match pair.as_rule() {
                Rule::label => self.current_label = Some(pair.as_str().to_owned()),
                Rule::dir if pair.as_str().starts_with(".macro") => {
                    let mac = self.parse_macro(pair, &mut lines)?;
                    if !second_pass {
                        self.declare_macro(mac)?;
                    }
                    self.current_label = None;
                }
                Rule::dir | Rule::inst => self.parse_line(pair, second_pass)?,
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }
        // if there's a dangling label, treat it as a code label
        if !second_pass {
            self.declare_label(self.program.len() as u64)?;
        }
        Ok(())
    }
}
