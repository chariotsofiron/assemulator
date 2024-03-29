mod opcode;

use assemulator::{mask, run, Cpu, Port, State, Token};
use opcode::Opcode;
use std::num::Wrapping;

type Register = assemulator::Register<8>;
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

// fn memory(op: u16, a: Register, b: Register, imm: u64) -> Result<u16, String> {
//     // if the address could be interpreted as a port after halving, then don't halve it
//     let imm = match Port::try_from((imm / 2) as usize) {
//         Ok(_) => imm,
//         Err(_) => ((imm as i64) / 2) as u64,
//     };
//     Ok(op << 13 | (a.0 as u16) << 10 | (b.0 as u16) << 7 | mask::<u16>(imm, 7)?)
// }

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

    fn parse(tokens: Vec<Token<Self::Opcode, Self::Reg>>, address: u64) -> Result<Vec<u8>, String> {
        use Opcode::*;
        use Token::*;
        let instruction = match *tokens {
            [Op(Add), Reg(a), Reg(b), Reg(c)] => fmt1(0b000, a, b, c),
            [Op(Add), Reg(a), Reg(b), Imm(c)] => fmt2(0b001, a, b, c)?,
            [Op(Nand), Reg(a), Reg(b), Reg(c)] => fmt1(0b010, a, b, c),
            [Op(Lui), Reg(a), Imm(imm)] => fmt3(0b011, a, imm)?,
            [Op(Ld), Reg(a), Reg(b), Imm(c)] if b.0 == 0 => fmt2(0b100, a, b, c)?,
            [Op(Ld), Reg(a), Reg(b), Imm(c)] => fmt2(0b100, a, b, c / 2)?,
            [Op(St), Reg(a), Reg(b), Imm(c)] if b.0 == 0 => fmt2(0b101, a, b, c)?,
            [Op(St), Reg(a), Reg(b), Imm(c)] => fmt2(0b101, a, b, c / 2)?,
            [Op(Beq), Reg(a), Reg(b), Imm(c)] => {
                fmt2(0b110, a, b, (((c.wrapping_sub(address)) as i64) / 2) as u64)?
            }
            [Op(Jalr), Reg(a), Reg(b)] => fmt2(0b111, a, b, 0)?,
            // pseudo-ops
            [Op(Add), Reg(a), Imm(b)] => fmt2(0b001, a, a, b)?,
            [Op(Add), Reg(a), Reg(b)] => fmt1(0b000, a, a, b),
            _ => Err(format!("invalid instruction: {:?}", tokens))?,
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
        let addr = (self.regs[rb] + imm).0;

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
            // port load
            0b100 if rb == 0 => {
                let port = Port::try_from(usize::from(addr)).expect("invalid port");
                self.regs[ra] = Wrapping(self.ports.read_port(port));
            }
            // memory load
            0b100 => {
                self.regs[ra] = Wrapping(self.data[usize::from(addr)]);
            }
            // port store
            0b101 if rb == 0 => {
                let port = Port::try_from(usize::from(addr)).expect("invalid port");
                self.ports.write_port(port, self.regs[ra].0);
            }
            // memory store
            0b101 => {
                self.data[usize::from(addr)] = self.regs[ra].0;
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
        usize::from(usize::from(self.pc.0) < self.program.len())
    }
}

fn main() {
    run::<Risc16>();
}
