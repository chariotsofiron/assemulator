mod enums;

use super::{Cpu, Token};
use crate::port::{Port, PortAddress};
use crate::util::mask;
use enum_map::EnumMap;
use enums::{Mnemonic, Register};
use std::num::Wrapping;

type WordSize = u16;

#[derive(Default)]
pub struct Risc16 {
    /// Program counter
    pc: Wrapping<WordSize>,
    /// General purpose registers
    regs: EnumMap<Register, Wrapping<WordSize>>,
    program: Vec<u16>,
    data: Vec<WordSize>,
    ports: Port<WordSize>,
}

impl Cpu for Risc16 {
    type Opcode = Mnemonic;
    type Reg = Register;

    fn new(addr: u64, program: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            pc: Wrapping(addr as WordSize),
            program: program
                .chunks_exact(2)
                .map(|x| u16::from_be_bytes([x[0], x[1]]))
                .collect(),
            data: data
                .chunks_exact(2)
                .map(|x| u16::from_be_bytes([x[0], x[1]]))
                .collect(),
            ..Default::default()
        }
    }

    fn parse_tokens(
        tokens: Vec<Token<Self::Opcode, Self::Reg>>,
        address: u64,
    ) -> Result<Vec<u8>, String> {
        use Mnemonic::{Add, Beq, Jalr, Ld, Lui, Nand, St};
        use Token::{Imm, Inst, Reg};
        let instruction: u16 = match *tokens {
            [Inst(Add), Reg(ra), Reg(rb), Reg(rc)] => ra.to_a() | rb.to_b() | rc.to_c(),
            [Inst(Add), Reg(ra), Reg(rb), Imm(imm)] => {
                0b001 << 13 | ra.to_a() | rb.to_b() | mask::<u16>(imm, 7)?
            }
            [Inst(Nand), Reg(ra), Reg(rb), Reg(rc)] => {
                0b010 << 13 | ra.to_a() | rb.to_b() | rc.to_c()
            }
            [Inst(Lui), Reg(ra), Imm(imm)] => 0b011 << 13 | ra.to_a() | mask::<u16>(imm, 10)?,
            [Inst(St), Reg(ra), Reg(rb), Imm(imm)] => {
                0b100 << 13 | ra.to_a() | rb.to_b() | mask::<u16>(imm, 7)?
            }
            [Inst(Ld), Reg(ra), Reg(rb), Imm(imm)] => {
                0b101 << 13 | ra.to_a() | rb.to_b() | mask::<u16>(imm, 7)?
            }
            [Inst(Beq), Reg(ra), Reg(rb), Imm(imm)] => {
                0b110 << 13 | ra.to_a() | rb.to_b() | mask::<u16>((imm - address) / 2, 7)?
            }
            [Inst(Jalr), Reg(ra), Reg(rb)] => 0b111 << 13 | ra.to_a() | rb.to_b(),
            _ => Err("Invalid instruction".to_owned())?,
        };
        Ok(instruction.to_be_bytes().to_vec())
    }

    fn step(&mut self) -> usize {
        let inst = self.program[usize::from(self.pc.0)];
        self.pc += 1;

        let opcode = inst >> 13;
        let ra = Register::from(inst >> 10 & 0b111);
        let rb = Register::from(inst >> 7 & 0b111);
        let rc = Register::from(inst & 0b111);
        // sign-extend 7-bit immediate
        let imm = Wrapping((inst & 0x7f ^ 0x40) - 0x40);

        match opcode {
            // add
            0b000 => self.regs[ra] = self.regs[rb] + self.regs[rc],
            // add immediate
            0b001 => self.regs[ra] = self.regs[rb] + imm,
            // bitwise nand
            0b010 => self.regs[ra] = !(self.regs[rb] & self.regs[rc]),
            // load upper immediate
            0b011 => self.regs[ra] = Wrapping(inst << 6),
            // store
            0b100 => {
                let addr = (self.regs[rb] + imm).0;
                match PortAddress::try_from(usize::from(addr)) {
                    Ok(addr) => self.ports.write_port(addr, self.regs[ra].0),
                    Err(_) => self.data[usize::from(addr)] = self.regs[ra].0,
                }
            }
            // load
            0b101 => {
                let addr = (self.regs[rb] + imm).0;
                self.regs[ra] = match PortAddress::try_from(usize::from(addr)) {
                    Ok(addr) => Wrapping(self.ports.read_port(addr)),
                    Err(_) => Wrapping(self.data[usize::from(addr)]),
                }
            }
            // branch if equal
            0b110 => {
                if self.regs[ra] == self.regs[rb] {
                    self.pc += imm - Wrapping(1);
                }
            }
            // jump and link register
            0b111 => {
                self.regs[ra] = self.pc;
                self.pc = self.regs[rb];
            }
            _ => unreachable!(),
        }
        self.regs[Register::R0] = Wrapping(0); // R0 is always 0
        usize::from(usize::from(self.pc.0) < self.program.len())
    }
}
