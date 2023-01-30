mod enums;
use super::{Cpu, Token};
use crate::port::{Port, PortAddress};
use crate::util::mask;
use enum_map::EnumMap;
use enums::Mnemonic;
use enums::Register;
use std::num::Wrapping;
type WordSize = u8;

#[derive(Default)]
pub struct Mira {
    pc: Wrapping<WordSize>,
    regs: EnumMap<Register, Wrapping<WordSize>>,
    x: bool,
    program: Vec<u8>,
    
    data: Vec<WordSize>,
    ports: Port<WordSize>,
}

impl Cpu for Mira {
    type Opcode = Mnemonic;
    type Reg = Register;

    fn new(addr: u64, program: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            pc: Wrapping(addr as WordSize),
            program,
            data,
            ..Default::default()
        }
    }
    fn parse_tokens(
        tokens: Vec<Token<Self::Opcode, Self::Reg>>,
        address: u64,
    ) -> Result<Vec<u8>, String> {
        use Mnemonic::{Adc, Add};
        use Token::{Imm, Inst, Reg};
        let instruction: u16 = match *tokens {
            _ => Err("Invalid instruction".to_owned())?,
        };
        Ok(instruction.to_le_bytes().to_vec())
    }

    fn step(&mut self) -> usize {
        0
    }
}
