use crate::cpu::{Cpu, Token};
use regex::Regex;
mod expression;
mod macros;

fn parse_line<'a>(line: &'a str) -> (Option<&'a str>, Vec<&'a str>) {
    macro_rules! pat {
        ($input:expr) => {{
            Regex::new(&format!($input)).unwrap()
        }};
    }

    let pattern = {
        let spc = pat!(r"\s*");
        let label = pat!(r"(?:(\w+):)");
        let control = pat!(r"(.?\w+)");
        let arg = pat!(r"[\w\d+-]+");
        let args = pat!(r"({arg}(?:,{spc}{arg})*)");
        let inst = pat!(r"(?:{control}{spc}{args})");
        let comment = pat!(r"(?:;.*)");
        pat!(r"^{spc}{label}?{spc}{inst}?{spc}{comment}?$")
    };

    let mut inst = vec![];
    if let Some(cap) = pattern.captures(line) {
        let label = cap.get(1).map(|x| x.as_str());

        if let Some(opcode) = cap.get(2) {
            inst.push(opcode.as_str());

            if let Some(args) = cap.get(3) {
                let arg_list = args.as_str().split(',').into_iter().map(|x| x.trim());
                inst.extend(arg_list);
            }
        }
        (label, inst)
    } else {
        panic!("Couldn't parse: {}", line)
    }
}

pub struct Assembler<T> {
    program: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Cpu> Assembler<T> {
    pub fn new(text: &str) -> Self {
        Self {
            program: text.to_owned(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn assemble(&mut self) {}

    ///
    /// # Arguments
    ///
    /// * `arg` - The argument to parse
    /// * `address` - The address of this instruction
    fn parse_arg(&self, arg: &str, address: u64) -> Result<Token<T::Reg>, String> {
        // it's either a register or an expression
        T::Reg::try_from(arg)
            .map(Token::Reg)
            .or_else(|_| Ok(Token::Imm(0)))
    }

    fn find_macros(&self) {
        for line in self.program.lines() {
            let (label, inst) = parse_line(line);
            if let Some(&command) = inst.first() {
                if command == ".macro" {}
            }
        }
    }

    fn first_pass(&self) -> Result<(), String> {
        let mut current_label = None;
        let mut address = 0;
        for line in self.program.lines() {
            println!("line: {line}");
            let (label, inst) = parse_line(line);
            if let Some(label) = label {
                current_label = Some(label);
            }
            if let Some(opcode) = inst.first() {
                let mut instruction = vec![Token::<T>::Op(*opcode)];

                let blah = inst[1..]
                    .iter()
                    .map(|&x| self.parse_arg(x, address)?)
                    .collect::<Vec<_>>();
                let i: i32 = blah;
                // instruction.extend(

                // );
                // println!("pieces: {:?}", instruction);
            }
        }
        Ok(())
    }
}
