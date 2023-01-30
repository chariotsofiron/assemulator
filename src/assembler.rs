use crate::cpus::{Cpu, Token};
use crate::port::PortAddress;
use crate::util::{mask, parse_int};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::collections::HashMap;
use std::ops::Neg;
use strum::IntoEnumIterator;

/// Pest parser for the assembler.
#[derive(Parser)]
#[grammar = "./src/grammar.pest"]
struct AsmParser;

/// Parses source into instructions.
#[derive(Debug)]
pub struct Assembler {
    /// Statically defined ram data.
    pub data: Vec<u8>,

    /// Stores assembled instructions.
    pub program: Vec<u8>,

    /// Symbol table, holds constants.
    /// Constants may be stored as 2s complement and depends on context.
    pub symbols: HashMap<String, u64>,
}

impl Default for Assembler {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            program: Vec::new(),
            symbols: PortAddress::iter()
                .map(|x| (format!("{x:?}").to_ascii_lowercase(), usize::from(x) as u64))
                .collect(),
        }
    }
}

/// Removes surrounding quotes and substitutes escape sequences in a string.
fn remove_quotes(s: &str) -> String {
    s.replace("\\n", "\n")
        .trim_matches(&['"', '\''][..])
        .trim_end_matches(&['"', '\''][..])
        .to_owned()
}

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

impl Assembler {
    /// Parses a constant argument
    /// Replaces a label, char, or int to an int
    /// Tries to fit it inside T
    fn parse_constant<T>(&self, arg: &Pair<Rule>) -> Result<T, String>
    where
        T: TryFrom<u64> + Copy,
    {
        let value: u64 = match arg.as_rule() {
            Rule::num => parse_int(arg.as_str()).map_err(|err| format_error(arg, &err)),
            Rule::ident => self
                .symbols
                .get(arg.as_str())
                .copied()
                .ok_or_else(|| format_error(arg, "Undefined label")),
            Rule::full_char => remove_quotes(arg.as_str())
                .bytes()
                .next()
                .map(u64::from)
                .ok_or_else(|| format_error(arg, "Empty character")),
            _ => Err(format_error(arg, "Unexpected token")),
        }?;

        let n_bits = 8 * std::mem::size_of::<T>();
        mask(value, n_bits).map_err(|err| format_error(arg, &err))
    }

    /// Parses an argument into a token.
    ///
    /// # Arguments
    ///
    /// * `arg` - The argument to parse into a `Token`.
    /// * `lookup_labels` - Whether to lookup labels or replace with 0. Used on
    ///  first pass when labels are not yet defined.
    /// * `address` - The address of the instruction.
    fn parse_arg<T: Cpu>(
        &self,
        arg: &Pair<Rule>,
        lookup_labels: bool,
        address: u64,
    ) -> Result<Token<T::Opcode, T::Reg>, String> {
        // anonymous labels
        if lookup_labels
            && (arg.as_str().chars().all(|c| c == '-') || arg.as_str().chars().all(|c| c == '+'))
        {
            let offset = if arg.as_str().starts_with('-') {
                (arg.as_str().len() as isize).neg()
            } else {
                arg.as_str().len() as isize
            };
            return self
                .get_anonymous_label(address, offset)
                .map(Token::Imm)
                .map_err(|err| format_error(arg, &err));
        }

        // everything else
        arg.as_str()
            // try to parse as opcode
            .parse::<T::Opcode>()
            .map(Token::Inst)
            .or_else(|_| {
                arg.as_str()
                    // try to parse as register
                    .parse::<T::Reg>()
                    .map(Token::Reg)
                    // otherwise try to parse label / constant
                    .or_else(|_| {
                        if lookup_labels {
                            self.parse_constant(arg).map(Token::Imm)
                        } else {
                            Ok(Token::Imm(0))
                        }
                    })
            })
    }

    /// Parse a directive declaration.
    fn handle_directive(&mut self, dir_cmd: &Pair<Rule>, labels: &[&str]) -> Result<(), String> {
        let mut pairs = dir_cmd.clone().into_inner();
        let directive = pairs
            .next()
            .ok_or_else(|| format_error(dir_cmd, "Expected directive"))?;
        match directive.as_str() {
            "i8" => {
                for pair in pairs {
                    self.data.push(self.parse_constant(&pair)?);
                }
            }
            "i16" => {
                for pair in pairs {
                    self.data
                        .extend_from_slice(&self.parse_constant::<u16>(&pair)?.to_le_bytes());
                }
            }
            "strz" => {
                // null-terminated string
                for pair in pairs {
                    self.data.extend(remove_quotes(pair.as_str()).as_bytes());
                    self.data.push(0);
                }
            }
            "set" => {
                let value = pairs
                    .next()
                    .ok_or_else(|| format_error(dir_cmd, "Expected value"))?;
                for label in labels {
                    self.symbols
                        .insert((*label).to_owned(), self.parse_constant(&value)?);
                }
            }
            "fill" => {
                let len = self.parse_constant(&pairs.next().ok_or_else(|| {
                    format_error(dir_cmd, "Expected argument for fill directive")
                })?)?;
                match pairs.next() {
                    Some(pair) => {
                        let val = self.parse_constant::<u8>(&pair)?;
                        self.data.extend(std::iter::repeat(val).take(len));
                    }
                    None => self.data.extend(std::iter::repeat(0).take(len)),
                }
            }
            _ => return Err(format_error(&directive, "Unknown directive")),
        }
        Ok(())
    }

    /// Performs the first pass of the assembler, declaring labels and handling directives
    fn first_pass<T: Cpu>(&mut self, pairs: Pairs<Rule>) -> Result<(), String> {
        // we cache the labels because its address depends on whether the next token is
        // an instruction or data directive
        let mut current_labels: Vec<&str> = Vec::new();
        let mut address: u64 = 0;
        for pair in pairs {
            match pair.as_rule() {
                Rule::label => {
                    if self.symbols.contains_key(pair.as_str()) {
                        return Err(format_error(&pair, "Duplicate label"));
                    }
                    current_labels.push(pair.as_str());
                }
                Rule::EOI => {
                    self.symbols
                        .extend(current_labels.drain(..).map(|x| (x.to_owned(), address)));
                }
                Rule::inst => {
                    self.symbols.extend(current_labels.drain(..).map(|x| {
                        (
                            if x == "@" {
                                format!("@{address}")
                            } else {
                                x.to_owned()
                            },
                            address,
                        )
                    }));

                    let tokens = pair
                        .clone()
                        .into_inner()
                        .map(|x| self.parse_arg::<T>(&x, false, address))
                        .collect::<Result<Vec<_>, _>>()?;
                    let inst_bytes = T::parse_tokens(tokens, address)
                        .map_err(|err| format_error(&pair, &err))?;
                    address += inst_bytes.len() as u64;
                }
                Rule::dir => {
                    self.symbols.extend(
                        current_labels
                            .drain(..)
                            .map(|x| (x.to_owned(), self.data.len() as u64)),
                    );
                    self.handle_directive(&pair, &current_labels)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Gets the label
    /// negative -> backward
    /// 0  -> returns i
    /// positive -> forward
    fn get_anonymous_label(&self, address: u64, offset: isize) -> Result<u64, String> {
        if offset == 0 {
            return Ok(address);
        }

        let mut anon_labels: Vec<_> = self
            .symbols
            .iter()
            .filter(|(key, &value)| {
                key.starts_with('@')
                    && if offset < 0 {
                        value < address
                    } else {
                        value > address
                    }
            })
            .map(|(_, v)| *v)
            .collect();

        anon_labels.sort_unstable(); // sort in ascending order
        if offset < 0 {
            anon_labels.reverse();
        }
        anon_labels
            .into_iter()
            .nth(offset.unsigned_abs() - 1)
            .ok_or_else(|| "Anon label landed outside range".to_owned())
    }

    /// Generates the program by parsing instructions and substituting symbols.
    fn second_pass<T: Cpu>(&mut self, pairs: Pairs<Rule>) -> Result<(), String> {
        for pair in pairs.filter(|x| matches!(x.as_rule(), Rule::inst)) {
            // parse the instruction into tokens by translating args to immediates
            let tokens = pair
                .clone()
                .into_inner()
                .map(|x| self.parse_arg::<T>(&x, true, self.program.len() as u64))
                .collect::<Result<Vec<_>, String>>()?;

            // parse the tokens into bytes
            let inst = T::parse_tokens(tokens, self.program.len() as u64)?;

            self.program.extend(inst.iter());
        }
        Ok(())
    }

    /// Assembles the given text.
    pub fn assemble<T: Cpu>(&mut self, text: &str) -> Result<(), String> {
        let pairs = AsmParser::parse(Rule::lines, text).map_err(|e| e.to_string())?;
        self.first_pass::<T>(pairs.clone())?;
        self.second_pass::<T>(pairs)?;
        Ok(())
    }
}
