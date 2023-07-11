use super::{Cpu, Token};
use crate::cpu::reg;
use crate::port::Port;
use crate::{port::State, util::mask};
use core::num::Wrapping;
use opcode::Opcode;
pub mod opcode;

pub type Register = reg::Register<8>;
type Word = u16;

#[derive(Default)]
pub struct Risc16 {
    pc: Wrapping<Word>,
    regs: [Wrapping<Word>; 8],
    program: Vec<u16>,
    data: Vec<Word>,
    ports: State<Word>,
}

fn fmt1(op: u16, a: Register, b: Register, c: Register) -> u16 {
    op << 13 | (a.0 as u16) << 10 | (b.0 as u16) << 7 | (c.0 as u16)
}

fn memory(op: u16, a: Register, b: Register, imm: u64) -> Result<u16, String> {
    // if the address could be interpreted as a port after halving, then don't halve it
    let imm = match Port::try_from((imm / 2) as usize) {
        Ok(_) => imm,
        Err(_) => ((imm as i64) / 2) as u64,
    };
    Ok(op << 13 | (a.0 as u16) << 10 | (b.0 as u16) << 7 | mask::<u16>(imm, 7)?)
}

fn fmt2(op: u16, a: Register, b: Register, imm: u64) -> Result<u16, String> {
    Ok(op << 13 | (a.0 as u16) << 10 | (b.0 as u16) << 7 | mask::<u16>(imm, 7)?)
}

fn fmt3(op: u16, a: Register, imm: u64) -> Result<u16, String> {
    Ok(op << 13 | (a.0 as u16) << 10 | mask::<u16>(imm, 10)?)
}

impl Cpu for Risc16 {
    type Opcode = Opcode;
    type Reg = Register;

    fn new(pc: u64, program: Vec<u8>, data: Vec<u8>) -> Self {
        Risc16 {
            pc: Wrapping(pc as Word),
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

    fn parse(
        tokens: Vec<Token<Self::Opcode, Self::Reg>>,
        address: usize,
    ) -> Result<Vec<u8>, String> {
        use Opcode::*;
        use Token::{Imm, Op, Reg};
        let instruction = match *tokens {
            [Op(Add), Reg(a), Reg(b), Reg(c)] => fmt1(0b000, a, b, c),
            [Op(Add), Reg(a), Reg(b), Imm(c)] => fmt2(0b001, a, b, c)?,
            [Op(Nand), Reg(a), Reg(b), Reg(c)] => fmt1(0b010, a, b, c),
            [Op(Lui), Reg(a), Imm(imm)] => fmt3(0b011, a, imm)?,
            [Op(Ld), Reg(a), Reg(b), Imm(c)] => memory(0b100, a, b, c)?,
            [Op(St), Reg(a), Reg(b), Imm(c)] => memory(0b101, a, b, c)?,
            [Op(Beq), Reg(a), Reg(b), Imm(c)] => fmt2(
                0b110,
                a,
                b,
                (((c.wrapping_sub(address as u64)) as i64) / 2) as u64,
            )?,
            [Op(Jalr), Reg(a), Reg(b)] => fmt2(0b111, a, b, 0)?,
            ref x => Err(format!("Invalid instruction: {x:?}"))?,
        };
        Ok(instruction.to_be_bytes().to_vec())
    }

    fn step(&mut self) -> usize {
        if usize::from(self.pc.0) >= self.program.len() {
            return 0;
        }
        let inst = self.program[usize::from(self.pc.0)];
        self.pc += 1;

        // decode instruction
        let opcode = inst >> 13;
        let ra = usize::from(inst >> 10 & 0b111);
        let rb = usize::from(inst >> 7 & 0b111);
        let rc = usize::from(inst & 0b111);
        // sign-extend 7-bit immediate
        let imm = Wrapping((inst & 0x7f ^ 0x40).wrapping_sub(0x40));

        // if opcode == 0b001 || opcode == 0b100 || opcode == 0b101 || opcode == 0b110 {
        //     println!("{}: {:#05b}, r{}, r{}, {}", self.pc.0 - 1, opcode, ra, rb, imm.0);
        // } else {
        //     println!("{}, {:#05b}, r{}, r{}, r{}", self.pc.0 - 1, opcode, ra, rb, rc);
        // }

        match opcode {
            // add
            0b000 => self.regs[ra] = self.regs[rb] + self.regs[rc],
            // add immediate
            0b001 => self.regs[ra] = self.regs[rb] + imm,
            // bitwise nand
            0b010 => self.regs[ra] = !(self.regs[rb] & self.regs[rc]),
            // load upper immediate
            0b011 => self.regs[ra] = Wrapping(inst << 6),
            // load
            0b100 => {
                let addr = (self.regs[rb] + imm).0;
                self.regs[ra] = match Port::try_from(usize::from(addr)) {
                    Ok(addr) => Wrapping(self.ports.read_port(addr)),
                    Err(_) => Wrapping(self.data[usize::from(addr)]),
                }
            }
            // store
            0b101 => {
                let addr = (self.regs[rb] + imm).0;
                match Port::try_from(usize::from(addr)) {
                    Ok(addr) => self.ports.write_port(addr, self.regs[ra].0),
                    Err(_) => self.data[usize::from(addr)] = self.regs[ra].0,
                }
            }
            // branch if equal
            0b110 => {
                if self.regs[ra] == self.regs[rb] {
                    self.pc = self.pc + imm - Wrapping(1);
                }
            }
            // jump and link register
            0b111 => {
                self.regs[ra] = self.pc;
                self.pc = self.regs[rb];
            }
            _ => unreachable!(),
        }
        self.regs[0] = Wrapping(0); // R0 is always 0
                                    // println!("regs: {:?}", self.regs);
        usize::from(usize::from(self.pc.0) < self.program.len())
    }
}
