//! Assembler for the CPU.
//!
//! first pass directives: include, macro
use self::macros::Macro;
use crate::{
    cpu::{Cpu, Token},
    port::Port,
    util::{mask, parse_int},
};
use regex::Regex;
use std::{collections::HashMap, iter::Zip, ops::RangeFrom};
mod expression;
mod macros;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;

lazy_static! {
        static ref PATTERN: Regex =  {
            macro_rules! pat {
                ($input:expr) => {{
                    Regex::new(&format!($input)).unwrap()
                }};
            }

            let spc = pat!(r"\s*");
            let label = pat!(r"(?:(\w+):)");
            let control = pat!(r"(\.?\w+)");
            // this will still need to be parsed
            let arg = pat!(r"\$?[\w\d+\-][\w\d+\-\s]*");
            // comma-separated list of arguments
            let args = pat!(r"({arg}(?:,{spc}{arg})*)");
            let inst = pat!(r"(?:{control}(?:\s+{args})?)");
            let comment = pat!(r"(?:;.*)");
            pat!(r"^{spc}{label}?{spc}{inst}?{spc}{comment}?$")
        };

        static ref LABEL: Regex = Regex::new(r"^\s*(\w+):").unwrap();
}

/// Removes surrounding quotes and substitutes escape sequences in a string.
fn remove_quotes(s: &str) -> String {
    s.replace("\\n", "\n")
        .trim_matches(&['"', '\''][..])
        .trim_end_matches(&['"', '\''][..])
        .to_owned()
}

// fn parse_label<'a>(line: &'a str) -> Option<&'a str> {
//     let label_pttn = Regex::new(r"^\s*(\w+):").unwrap();
//     if let Some(cap) = label_pttn.captures(line) {
//         cap.get(1).map(|x| x.as_str())
//     } else {
//         None
//     }
// }

// /// Tokenizes a line of the assembler into a vector of opcode and arguments.
// fn tokenize_line<'a>(line: &'a str) -> Result<Vec<&'a str>, String> {
//     let mut inst = Vec::new();
//     if let Some(cap) = PATTERN.captures(line) {
//         if let Some(opcode) = cap.get(2) {
//             inst.push(opcode.as_str());
//             if let Some(args) = cap.get(3) {
//                 let arg_list = args.as_str().split(',').into_iter().map(|x| x.trim());
//                 inst.extend(arg_list);
//             }
//         }
//         Ok(inst)
//     } else {
//         Err(format!("invalid line: {line}"))
//     }
// }

// /// Generic over a specific CPU
// pub struct Assembler<'a, Cpu> {
//     lines: Zip<RangeFrom<usize>, std::str::Lines<'a>>,
//     /// Statically defined ram data.
//     pub data: Vec<u8>,
//     /// Program bytes.
//     pub program: Vec<u8>,
//     /// Symbol table, holds constants.
//     /// Constants may be stored as 2s complement and depends on context.
//     pub symbols: HashMap<String, u64>,
//     /// Macro name -> Macro
//     macros: HashMap<String, Macro>,
//     current_label: Option<String>,
//     line_num: usize,
//     phantom: std::marker::PhantomData<Cpu>,
// }

// impl<'a, T: Cpu> Assembler<'a, T> {
//     /// Creates a new assembler for an assembly program.
//     pub fn new(file: &str) -> Self {
//         let symbols: HashMap<String, u64> = Port::iter()
//             .map(|x| {
//                 (
//                     format!("{:?}", x).to_ascii_lowercase(),
//                     usize::from(x) as u64,
//                 )
//             })
//             .collect();
//         Self {
//             lines: (1..).zip(text.lines()),
//             data: vec![],
//             program: Default::default(),
//             symbols,
//             macros: Default::default(),
//             current_label: None,
//             line_num: 0,
//             phantom: std::marker::PhantomData,
//         }
//     }

//     pub fn assemble(&mut self) -> Result<(), String> {
//         let tmp = self.lines.clone();
//         self.passover(false)
//             .map_err(|err| format!("line {}: {}", self.line_num, err.to_string()))?;
//         self.program.clear();
//         self.data.clear();
//         self.lines = tmp;
//         self.passover(true)
//             .map_err(|err| format!("line {}: {}", self.line_num, err.to_string()))?;

//         Ok(())
//     }

//     ///
//     /// # Arguments
//     ///
//     /// * `arg` - The argument to parse
//     /// * `address` - The address of this instruction
//     fn parse_arg(&self, arg: &str, second_pass: bool) -> Result<Token<T::Opcode, T::Reg>, String> {
//         // it's either a register or an expression
//         if let Ok(reg) = T::Reg::try_from(arg) {
//             Ok(Token::Reg(reg))
//         } else {
//             self.parse_constant(arg, second_pass).map(Token::Imm)
//         }
//     }

//     fn parse_constant<U>(&self, arg: &str, second_pass: bool) -> Result<U, String>
//     where
//         U: TryFrom<u64> + Copy,
//     {
//         // regex to match a character with surrounding single quotes
//         let ch_pttn = Regex::new(r"'([[:ascii:]])'").unwrap();
//         let value = if let Ok(val) = parse_int(arg) {
//             val
//         } else if let Some(cap) = ch_pttn.captures(arg) {
//             let ch = cap[1].chars().next().unwrap();
//             u64::from(ch)
//         } else if !second_pass {
//             0
//         } else if let Some(&val) = self.symbols.get(arg) {
//             val
//         } else {
//             return Err(format!("invalid constant: {arg}"));
//         };

//         let n_bits = 8 * std::mem::size_of::<T>();
//         mask(value, n_bits).map_err(|err| format!("constant out of range: {err}"))
//     }

//     /// Parses a macro.
//     ///
//     /// # Arguments
//     ///
//     /// * `args` - The arguments to the macro
//     fn declare_macro(&mut self, args: &[&str]) -> Result<Macro, String> {
//         let mut instructions = Vec::new();
//         while let Some((line_num, line)) = self.lines.next() {
//             self.line_num = line_num;
//             let inst = tokenize_line(line).map_err(|err| format!("{line_num}: {err}"))?;
//             if inst.first() == Some(&".endm") {
//                 break;
//             }
//             instructions.push(line.to_owned());
//         }

//         Ok(Macro::new(
//             args.into_iter().map(|&x| x.to_owned()).collect::<Vec<_>>(),
//             instructions,
//         ))
//     }

//     /// Handles an assembler directive.
//     fn handle_directive(
//         &mut self,
//         label: Option<String>,
//         directive: &str,
//         args: &[&str],
//         second_pass: bool,
//     ) -> Result<(), String> {
//         match directive {
//             ".i8" => {
//                 if !second_pass {
//                     self.declare_label(label, self.data.len() as u64)?;
//                 }
//                 for arg in args {
//                     self.data.push(self.parse_constant(arg, second_pass)?);
//                 }
//             }
//             ".strz" => {
//                 if !second_pass {
//                     self.declare_label(label, self.data.len() as u64)?;
//                 }
//                 // null-terminated string
//                 for arg in args {
//                     self.data.extend(remove_quotes(arg).as_bytes());
//                     self.data.push(0);
//                 }
//             }
//             ".set" => self.declare_label(label, self.parse_constant(args[0], second_pass)?)?,
//             ".include" => {
//                 for arg in args {
//                     let filename = remove_quotes(arg);
//                     let text = std::fs::read_to_string(&filename).map_err(|x| x.to_string())?;
//                 }
//             }
//             _ => return Err(format!("unknown directive: {directive}")),
//         }
//         Ok(())
//     }

//     fn declare_label(&mut self, label: Option<String>, value: u64) -> Result<(), String> {
//         if let Some(label) = label {
//             if self.symbols.contains_key(&label) {
//                 return Err(format!("label already defined: {label}"));
//             }
//             self.symbols.insert(label, value);
//         }
//         Ok(())
//     }

//     // fn blahblah(&mut self, tokens: &[&str]) {
//     //     match tokens {
//     //         [".macro", args @ ..] => {}
//     //         [".i8", args @ ..] => {}
//     //         [".strz", args @ ..] => {}
//     //         [".set", args @ ..] => {}
//     //         [".include", args @ ..] => {}
//     //         [opcode, args @ ..] => {
//     //             let mut pieces: Vec<Token<T::Opcode, T::Reg>> =
//     //                 vec![Token::Op(T::Opcode::try_from(opcode)?)];
//     //             for &arg in args {
//     //                 pieces.push(self.parse_arg(arg, second_pass)?);
//     //             }
//     //         }
//     //         _ => {}
//     //     }
//     // }

//     fn inside(&mut self, line: &str, second_pass: bool) -> Result<(), String> {
//         if let Some(label) = parse_label(line) {
//             self.current_label = Some(label.to_owned());
//         }

//         let tokens = tokenize_line(line)?;
//         match tokens.first() {
//             // macro definition
//             Some(&".macro") => {
//                 let mac = self.declare_macro(&tokens[1..])?;
//                 if second_pass {
//                     return Ok(());
//                 }
//                 if let Some(label) = std::mem::replace(&mut self.current_label, None) {
//                     if self.macros.contains_key(&label) {
//                         return Err(format!("macro already defined: {label}"));
//                     }
//                     self.macros.insert(label, mac);
//                 }
//             }
//             // macro invocation
//             Some(&x) if self.macros.contains_key(x) => {
//                 for macro_line in self.macros[x].expand(&tokens[1..]) {
//                     self.inside(&macro_line, second_pass)?;
//                 }
//             }
//             // directive
//             Some(&x) if x.starts_with('.') => {
//                 let label = std::mem::replace(&mut self.current_label, None);
//                 self.handle_directive(label, x, &tokens[1..], second_pass)?;
//             }
//             // instruction
//             Some(&x) => {
//                 let mut pieces: Vec<Token<T::Opcode, T::Reg>> =
//                     vec![Token::Op(T::Opcode::try_from(x)?)];
//                 for &arg in &tokens[1..] {
//                     pieces.push(self.parse_arg(arg, second_pass)?);
//                 }
//                 let bytes = T::parse(pieces, self.program.len())?;
//                 if !second_pass {
//                     if let Some(label) = std::mem::replace(&mut self.current_label, None) {
//                         if self.symbols.contains_key(&label) {
//                             return Err(format!("label already defined: {label}"));
//                         }
//                         self.symbols.insert(label, self.program.len() as u64);
//                     }
//                 }
//                 self.program.extend(bytes);
//             }
//             None => {}
//         }
//         Ok(())
//     }

//     /// Declare labels
//     /// Declare and expand macros
//     fn passover(&mut self, second_pass: bool) -> Result<(), String> {
//         while let Some((line_num, line)) = self.lines.next() {
//             self.line_num = line_num;
//             self.inside(line, second_pass)?;
//         }
//         Ok(())
//     }
// }

/// Tokenizes a line of the assembler into a vector of opcode and arguments.
fn tokenize_line<'a>(line: &'a str) -> Result<Vec<&'a str>, String> {
    let mut inst = Vec::new();
    if let Some(cap) = PATTERN.captures(line) {
        if let Some(opcode) = cap.get(2) {
            inst.push(opcode.as_str());
            if let Some(args) = cap.get(3) {
                let arg_list = args.as_str().split(',').into_iter().map(|x| x.trim());
                inst.extend(arg_list);
            }
        }
        Ok(inst)
    } else {
        Err(format!("Invalid line: {line}"))
    }
}

use std::path::Path;

pub struct Assembler2<T> {
    lines: std::str::Lines<'static>,
    line_num: usize,
    current_label: Option<String>,

    first_pass: bool,
    symbols: HashMap<String, u64>,
    address: usize,
    program: Vec<u8>,
    data: Vec<u8>,
    macros: HashMap<String, Macro>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Cpu> Assembler2<T> {
    /// Parse argument
    /// # Arguments
    ///
    /// * `arg` - The argument to parse
    /// * `address` - The address of this instruction
    fn parse_token(
        &self,
        arg: &str,
        second_pass: bool,
    ) -> Result<Token<T::Opcode, T::Reg>, String> {
        if let Ok(opcode) = T::Opcode::try_from(arg) {
            Ok(Token::Op(opcode))
        } else if let Ok(reg) = T::Reg::try_from(arg) {
            Ok(Token::Reg(reg))
        } else {
            self.parse_constant(arg, second_pass).map(Token::Imm)
        }
    }

    fn parse_constant<U>(&self, arg: &str, second_pass: bool) -> Result<U, String>
    where
        U: TryFrom<u64> + Copy,
    {
        // regex to match a character with surrounding single quotes
        let ch_pttn = Regex::new(r"'([[:ascii:]])'").unwrap();
        let value = if let Ok(val) = parse_int(arg) {
            val
        } else if let Some(cap) = ch_pttn.captures(arg) {
            let ch = cap[1].chars().next().unwrap();
            u64::from(ch)
        } else if !second_pass {
            0
        } else if let Some(&val) = self.symbols.get(arg) {
            val
        } else {
            return Err(format!("invalid constant: {arg}"));
        };

        let n_bits = 8 * std::mem::size_of::<T>();
        mask(value, n_bits).map_err(|err| format!("constant out of range: {err}"))
    }

    /// Handles an assembler directive.
    fn handle_directive(
        &mut self,
        label: Option<String>,
        directive: &str,
        args: &[&str],
        second_pass: bool,
    ) -> Result<(), String> {
        match directive {
            ".macro" => {
                // let mut instructions = Vec::new();
                // while let Some((line_num, line)) = self.lines.next() {
                //     self.line_num = line_num;
                //     let inst = tokenize_line(line).map_err(|err| format!("{line_num}: {err}"))?;
                //     if inst.first() == Some(&".endm") {
                //         break;
                //     }
                //     instructions.push(line.to_owned());
                // }

                // Ok(Macro::new(
                //     args.into_iter().map(|&x| x.to_owned()).collect::<Vec<_>>(),
                //     instructions,
                // ))
            }
            _ => return Err(format!("unknown directive: {directive}")),
        }
        Ok(())
    }

    // fn parse_line(&mut self, line: &str) -> Result<(), String> {
    //     match tokenize_line(line)?.as_slice() {
    //         [directive, args @ ..] if directive.starts_with('.') => {
    //             self.handle_directive(
    //                 self.current_label.clone(),
    //                 directive,
    //                 args,
    //                 self.first_pass,
    //             )?;
    //         }
    //         tokens => {
    //             let tokens = tokens
    //                 .iter()
    //                 .map(|x| self.parse_token(x, false))
    //                 .collect::<Result<Vec<_>, _>>()?;

    //             let bytes = T::parse(tokens, self.address)?;
    //         }
    //     }

    //     Ok(())
    // }

    pub fn pass(&mut self, file: &Path) -> Result<(), String> {
        let text = std::fs::read_to_string(file).map_err(|x| x.to_string())?;
        let x = text.lines().to_owned();

        // for (i, line) in text.lines().enumerate() {
        //     self.parse_line(line)
        //         .map_err(|err| format!("line {i}: {err}"))?;
        // }
        Ok(())
    }

    /// Updates line num and current_label
    fn parse<'a>(&mut self, line: &'a str) -> Result<Vec<&'a str>, String> {
        self.line_num += 1;

        let mut inst = Vec::new();
        if let Some(cap) = PATTERN.captures(line) {
            // parse label
            if let Some(label) = cap.get(1) {
                self.current_label = Some(label.as_str().to_owned());
            }
            // parse tokens
            if let Some(opcode) = cap.get(2) {
                inst.push(opcode.as_str());
                if let Some(args) = cap.get(3) {
                    let arg_list = args.as_str().split(',').into_iter().map(|x| x.trim());
                    inst.extend(arg_list);
                }
            }
            return Ok(inst);
        } else {
            return Err(format!("Invalid line: {line}"));
        }
    }

    pub fn parse_file(&mut self, filename: &str) -> Result<(), String> {
        let text = std::fs::read_to_string(filename)
            .map_err(|x| x.to_string())
            .unwrap();

        let mut lines = text.lines();
        self.line_num = 1;
        while let Some(line) = lines.next() {
            self.line_num += 1;
            match self.parse(line)?.as_slice() {
                [".macro", args @ ..] => {
                    // need to handle macros here so we can manually advance lines
                    let mut instructions = Vec::new();
                    while let Some(line) = self.lines.next() {
                        self.line_num += 1;
                        let inst = self.parse(line)?;
                        if inst.first() == Some(&".endm") {
                            break;
                        }
                        instructions.push(line.to_owned());
                    }

                    if let Some(label) = std::mem::replace(&mut self.current_label, None) {
                        if self.macros.contains_key(&label) {
                            return Err(format!("macro already defined: {label}"));
                        }
                        self.macros.insert(
                            label,
                            Macro::new(
                                args.into_iter().map(|&x| x.to_owned()).collect::<Vec<_>>(),
                                instructions,
                            ),
                        );
                    }
                }
                [directive, args @ ..] if directive.starts_with('.') => {
                    self.handle_directive(
                        self.current_label.clone(),
                        directive,
                        args,
                        self.first_pass,
                    )?;
                }
                tokens => {
                    let tokens = tokens
                        .iter()
                        .map(|x| self.parse_token(x, false))
                        .collect::<Result<Vec<_>, _>>()?;

                    let bytes = T::parse(tokens, self.address)?;
                }
            }
        }
        Ok(())
    }
}
