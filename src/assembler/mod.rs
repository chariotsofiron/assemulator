use crate::assembler::macros::Macro;
use crate::{
    cpu::{Cpu, Token},
    port::Port,
    util::{mask, parse_int},
};
mod macros;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::iter::Zip;
use std::ops::RangeFrom;
use std::path::Path;
use strum::IntoEnumIterator;

macro_rules! pat {
    ($input:expr) => {{
        Regex::new(&format!($input)).unwrap()
    }};
}

lazy_static! {
    static ref PATTERN: Regex =  {
        let spc = pat!(r"\s*");
        let ident = pat!(r"([a-zA-Z\.][\w\.]*)");
        let label = pat!(r"(?:{ident}:)");
        let arg = pat!(r"[^,]+");
        // comma-separated list of arguments
        let args = pat!(r"({arg}(?:,{spc}{arg})*)");
        let inst = pat!(r"(?:{ident}(?:\s+{args})?)");
        let comment = pat!(r"(?:;.*)");
        pat!(r"^{spc}{label}?{spc}{inst}?{spc}{comment}?$")
    };
    static ref CHAR: Regex = pat!(r"'([[:ascii:]])'");
}

/// Removes surrounding quotes and substitutes escape sequences in a string.
fn remove_quotes(s: &str) -> String {
    s.replace("\\n", "\n")
        .trim_matches(&['"', '\''][..])
        .trim_end_matches(&['"', '\''][..])
        .to_owned()
}

/// Valid for a specific CPU.
#[derive(Default)]
pub struct Assembler<T> {
    path: std::path::PathBuf,
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
        asm.include(&path, false)?;
        asm.data.clear();
        asm.program.clear();
        asm.include(&path, true)?;
        Ok(asm)
    }

    fn parse_constant<U>(&self, arg: &str, second_pass: bool) -> Result<U, String>
    where
        U: TryFrom<u64> + Copy,
    {
        let value = if let Ok(val) = parse_int(arg) {
            val
        } else if let Some(cap) = CHAR.captures(arg) {
            let ch = cap[1].chars().next().unwrap();
            u64::from(ch)
        } else if !second_pass {
            0
        } else if let Some(&val) = self.symbols.get(arg) {
            val
        } else {
            return Err(format!("undefined label: {arg}"));
        };
        let n_bits = 8 * std::mem::size_of::<U>();
        mask(value, n_bits).map_err(|err| format!("constant out of range: {err}"))
    }

    /// Updates current_label. Parses line into opcode and args.
    fn tokenize<'a>(&mut self, line: &'a str) -> Result<Vec<&'a str>, String> {
        let mut inst = Vec::new();
        if let Some(cap) = PATTERN.captures(line) {
            if let Some(label) = cap.get(1) {
                self.current_label = Some(label.as_str().to_owned());
            }
            if let Some(opcode) = cap.get(2) {
                inst.push(opcode.as_str());
                if let Some(args) = cap.get(3) {
                    let args = args.as_str().split(',').into_iter().map(|x| x.trim());
                    inst.extend(args);
                }
            }
            return Ok(inst);
        } else {
            return Err(format!("Invalid line: {line}"));
        }
    }

    fn parse_macro<'a>(
        &mut self,
        lines: &mut Zip<RangeFrom<usize>, std::str::Lines<'a>>,
        args: &[&str],
    ) -> Result<Macro, String> {
        // need to handle macros here so we can manually advance lines
        let mut instructions = Vec::new();
        while let Some((i, line)) = lines.next() {
            println!("{}: {line}", i + 1);
            let inst = self.tokenize(line).map_err(|err| format!("{i}: {err}"))?;
            if inst.first() == Some(&".endm") {
                break;
            }
            instructions.push(line.to_owned());
        }

        Ok(Macro::new(
            args.into_iter().map(|&x| x.to_owned()).collect::<Vec<_>>(),
            instructions,
        ))
    }

    fn decode_instruction(
        &self,
        opcode: &str,
        args: &[&str],
        second_pass: bool,
    ) -> Result<Vec<u8>, String> {
        let tokens = std::iter::once(Ok(Token::Op(T::Opcode::try_from(opcode)?)))
            .chain(args.iter().map(|x| match T::Reg::try_from(x) {
                Ok(reg) => Ok(Token::Reg(reg)),
                Err(_) => self.parse_constant(x, second_pass).map(Token::Imm),
            }))
            .collect::<Result<Vec<_>, _>>()?;
        let bytes = T::parse(tokens, self.program.len())?;
        Ok(bytes)
    }

    fn declare_macro(&mut self, value: Macro) -> Result<(), String> {
        if let Some(label) = std::mem::replace(&mut self.current_label, None) {
            if self.macros.contains_key(&label) || self.symbols.contains_key(&label) {
                return Err(format!("label already defined: {label}"));
            }
            self.macros.insert(label, value);
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

    fn parse_line(&mut self, tokens: &[&str], second_pass: bool) -> Result<(), String> {
        match *tokens {
            ref directive if directive.first().map_or(false, |x| x.starts_with('.')) => {
                // parse directive
                self.handle_directive(directive, second_pass)?;
            }
            [opcode, ref args @ ..] if self.macros.contains_key(opcode) => {
                // macro expansion
                for macro_line in self.macros[opcode].expand(args) {
                    let tokens = self.tokenize(&macro_line)?;
                    self.parse_line(&tokens, second_pass)?;
                }
            }
            [opcode, ref args @ ..] => {
                // instruction
                let bytes = self.decode_instruction(opcode, args, second_pass)?;
                self.program.extend(bytes);
                if !second_pass {
                    self.declare_label(self.program.len() as u64)?;
                }
            }
            ref x => return Err(format!("Invalid tokens: {:?}", x)),
        }
        Ok(())
    }

    /// Includes a file in the current assembly.
    fn include(&mut self, filename: &Path, second_pass: bool) -> Result<(), String> {
        let text = std::fs::read_to_string(filename).unwrap();
        let mut lines = (1usize..).zip(text.lines());
        while let Some((i, line)) = lines.next() {
            let tokens = self.tokenize(line).map_err(|err| format!("{i}: {err}"))?;
            if tokens.is_empty() {
                continue;
            }

            if tokens.first() == Some(&".macro") {
                let mac = self.parse_macro(&mut lines, &tokens[1..])?;
                if !second_pass {
                    self.declare_macro(mac)
                        .map_err(|err| format!("{i}: {err}"))?;
                }
            } else {
                self.parse_line(&tokens, second_pass)
                    .map_err(|err| format!("{i}: {err}"))?;
            }
        }
        Ok(())
    }

    fn handle_directive(&mut self, tokens: &[&str], second_pass: bool) -> Result<(), String> {
        match tokens {
            [".include", filename] => self.include(
                &self.path.parent().unwrap().join(remove_quotes(filename)),
                second_pass,
            )?,
            [".i8", ref args @ ..] => {
                if !second_pass {
                    self.declare_label(self.data.len() as u64)?;
                }
                for arg in args {
                    self.data.push(self.parse_constant(arg, second_pass)?);
                }
            }
            [".set", expression] => {
                // defined on the second pass, so need to be forward declared
                if second_pass {
                    self.declare_label(self.parse_constant(expression, second_pass)?)?;
                }
            }
            [".strz", ref args @ ..] => {
                // TODO fix this
                if !second_pass {
                    self.declare_label(self.data.len() as u64)?;
                }
                for arg in args {
                    self.data.extend(remove_quotes(arg).as_bytes());
                    self.data.push(0);
                }
            }
            toks => return Err(format!("Invalid directive: {:?}", toks)),
        }
        Ok(())
    }
}
