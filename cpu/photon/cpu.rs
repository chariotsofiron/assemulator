mod enums;

use assemulator::{mask, run};
use assemulator::{Cpu, Token};
use assemulator::{Port, State};

use enums::{Mnemonic, Register};

use std::num::Wrapping;
type WordSize = u8;

#[derive(Default)]
pub struct Photon {
    pc: Wrapping<WordSize>,
    regs: [Wrapping<WordSize>; 8],
    x: bool,
    program: Vec<u16>,
    data: Vec<WordSize>,
    ports: State<WordSize>,
}

impl Cpu for Photon {
    type Opcode = Mnemonic;
    type Reg = Register;

    fn new(addr: u64, program: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            pc: Wrapping((addr / 2) as WordSize),
            program: program
                .chunks_exact(2)
                .map(|x| u16::from_be_bytes([x[0], x[1]]))
                .collect(),
            data,
            ..Default::default()
        }
    }
    fn parse(tokens: Vec<Token<Self::Opcode, Self::Reg>>, _: usize) -> Result<Vec<u8>, String> {
        use Mnemonic::*;
        use Token::*;
        let inst: u16 = match *tokens {
            [Op(And), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b(),
            [Op(Or), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b001,
            [Op(Xor), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b010,
            [Op(Mov), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b011,
            [Op(Tst), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b100,
            [Op(Eq), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b101,
            [Op(Geq), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b110,
            [Op(Ges), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b111,
            [Op(And), Reg(ra), Imm(x)] => ra.to_a() | 0x800 | mask::<u16>(x, 8)?,
            [Op(Or), Reg(ra), Imm(x)] => ra.to_a() | 0x900 | mask::<u16>(x, 8)?,
            [Op(Xor), Reg(ra), Imm(x)] => ra.to_a() | 0xa00 | mask::<u16>(x, 8)?,
            [Op(Mov), Reg(ra), Imm(x)] => ra.to_a() | 0xb00 | mask::<u16>(x, 8)?,
            [Op(Tst), Reg(ra), Imm(x)] => ra.to_a() | 0xc00 | mask::<u16>(x, 8)?,
            [Op(Eq), Reg(ra), Imm(x)] => ra.to_a() | 0xd00 | mask::<u16>(x, 8)?,
            [Op(Geq), Reg(ra), Imm(x)] => ra.to_a() | 0xe00 | mask::<u16>(x, 8)?,
            [Op(Ges), Reg(ra), Imm(x)] => ra.to_a() | 0xf00 | mask::<u16>(x, 8)?,
            [Op(Add), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b01000,
            [Op(Addc), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b01001,
            [Op(Addx), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b01010,
            [Op(Addb), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b01011,
            [Op(Sub), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b01100,
            [Op(Subc), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b01101,
            [Op(Subx), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b01110,
            [Op(Subb), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b01111,
            [Op(Shl), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b10000,
            [Op(Shlc), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b10001,
            [Op(Shlx), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b10010,
            [Op(Shlb), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b10011,
            [Op(Shr), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b10100,
            [Op(Shrc), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b10101,
            [Op(Shrx), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b10110,
            [Op(Shrb), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b10111,
            [Op(Mvt), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b11000,
            [Op(Mvf), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b11001,
            [Op(Cad), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b11010,
            [Op(Csb), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b11011,
            [Op(Neg), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b11100,
            [Op(Swap), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b11101,
            [Op(Psh), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b11110,
            [Op(Pop), Reg(ra), Reg(rb)] => ra.to_a() | rb.to_b() | 0b11111,

            [Op(Bt), Imm(x)] => 0x1000 | mask::<u16>(x / 2, 8)?,
            [Op(Bf), Imm(x)] => 0x1100 | mask::<u16>(x / 2, 8)?,
            [Op(Jmp), Imm(x)] => 0x1200 | mask::<u16>(x / 2, 8)?,
            [Op(Jsr), Reg(ra), Imm(x)] => ra.to_a() | 0x1300 | mask::<u16>(x / 2, 8)?,
            [Op(Bt), Reg(ra), Imm(x)] => ra.to_a() | 0x1400 | mask::<u16>(x / 2, 8)?,
            [Op(Bf), Reg(ra), Imm(x)] => ra.to_a() | 0x1500 | mask::<u16>(x / 2, 8)?,
            [Op(Jmp), Reg(ra), Imm(x)] => ra.to_a() | 0x1600 | mask::<u16>(x / 2, 8)?,
            [Op(Btd), Reg(ra), Imm(x)] => ra.to_a() | 0x1700 | mask::<u16>(x / 2, 8)?,

            [Op(Ld), Reg(ra), Imm(x)] => ra.to_a() | 0x1800 | mask::<u16>(x, 8)?,
            [Op(St), Reg(ra), Imm(x)] => ra.to_a() | 0x1900 | mask::<u16>(x, 8)?,
            [Op(Pld), Reg(ra), Imm(x)] => ra.to_a() | 0x1a00 | mask::<u16>(x, 8)?,
            [Op(Pst), Reg(ra), Imm(x)] => ra.to_a() | 0x1b00 | mask::<u16>(x, 8)?,

            [Op(Ld), Reg(ra), Reg(rb), Imm(x)] => {
                ra.to_a() | 0x1c00 | rb.to_b() | mask::<u16>(x, 8)? & 0x1f
            }
            [Op(St), Reg(ra), Reg(rb), Imm(x)] => {
                ra.to_a() | 0x1d00 | rb.to_b() | mask::<u16>(x, 8)? & 0x1f
            }

            [Op(Add), Reg(ra), Reg(rb), Imm(x)] => {
                ra.to_a() | 0x1e00 | rb.to_b() | mask::<u16>(x, 5)?
            }

            // pseudo-ops
            [Op(Jsr), Imm(x)] => 0xf300 | mask::<u16>(x / 2, 8)?,
            [Op(Ret)] => 0xf600,
            [Op(Add), Reg(ra), Imm(x)] => ra.to_a() | 0x1e00 | mask::<u16>(x, 5)?,

            [Op(Shl), Reg(ra)] => ra.to_a() | ra.to_b() | 0x10,
            [Op(Shlc), Reg(ra)] => ra.to_a() | ra.to_b() | 0x11,
            [Op(Shlx), Reg(ra)] => ra.to_a() | ra.to_b() | 0x12,
            [Op(Shlb), Reg(ra)] => ra.to_a() | ra.to_b() | 0x13,
            [Op(Shr), Reg(ra)] => ra.to_a() | ra.to_b() | 0x14,
            [Op(Shrc), Reg(ra)] => ra.to_a() | ra.to_b() | 0x15,
            [Op(Shrx), Reg(ra)] => ra.to_a() | ra.to_b() | 0x16,
            [Op(Shrb), Reg(ra)] => ra.to_a() | ra.to_b() | 0x17,

            [Op(Neg), Reg(ra)] => ra.to_a() | ra.to_b() | 0b11100,
            // [Op(Swap), Reg(ra)] => Ok(Swap(ra, ra)),
            [Op(Ld), Reg(ra), Reg(rb)] => ra.to_a() | 0x1c00 | rb.to_b(),
            [Op(St), Reg(ra), Reg(rb)] => ra.to_a() | 0x1d00 | rb.to_b(),
            [Op(Pld), Imm(x)] => Register::A.to_a() | 0x1a00 | mask::<u16>(x, 8)?,
            [Op(Pst), Imm(x)] => Register::A.to_a() | 0x1b00 | mask::<u16>(x, 8)?,
            _ => Err("Invalid instruction".to_owned())?,
        };
        Ok(inst.to_be_bytes().to_vec())
    }

    fn step(&mut self) -> usize {
        let inst = self.program[usize::from(self.pc.0)];
        self.pc += 1;

        let ra = usize::from(inst.wrapping_shr(13));
        let rb = usize::from(inst.wrapping_shr(5));

        let z = if inst & 0x1c00 == 0x1c00 {
            // 5 bit sign extension
            (inst & 0x1f ^ 0x10) - 0x10
        } else {
            inst & 0xff
        };

        let imm = Wrapping(z as u8);

        if inst.wrapping_shr(8) & 0b11111 == 0 {
            match inst & 0b11111 {
                0x00 => self.regs[ra] = self.regs[ra] & self.regs[rb],
                0x01 => self.regs[ra] = self.regs[ra] | self.regs[rb],
                0x02 => self.regs[ra] = self.regs[ra] ^ self.regs[rb],
                0x03 => self.regs[ra] = self.regs[rb],
                0x04 => self.x = self.regs[ra] & self.regs[rb] != Wrapping(0),
                0x05 => self.x = self.regs[ra] == self.regs[rb],
                0x06 => self.x = self.regs[ra] >= self.regs[rb],
                0x07 => self.x = (self.regs[ra].0 as i8) >= (self.regs[rb].0 as i8),
                0x08 => self.regs[ra] = self.regs[ra] + self.regs[rb],
                0x09 => self.regs[ra] = self.regs[ra] + self.regs[rb] + Wrapping(u8::from(self.x)),
                0x0a => {
                    self.regs[ra] = self.regs[ra] + self.regs[rb];
                    self.x = self.regs[ra] < self.regs[rb];
                }
                0x0b => {
                    self.regs[ra] = self.regs[ra] + self.regs[rb] + Wrapping(u8::from(self.x));
                    self.x = self.regs[ra] < self.regs[rb];
                }
                0x0c => self.regs[ra] = self.regs[ra] - self.regs[rb],
                0x0d => self.regs[ra] = self.regs[ra] - self.regs[rb] + Wrapping(u8::from(self.x)),
                0x0e => {
                    self.regs[ra] = self.regs[ra] - self.regs[rb];
                    self.x = self.regs[ra] > self.regs[rb];
                }
                0x0f => {
                    self.regs[ra] = self.regs[ra] - self.regs[rb] + Wrapping(u8::from(self.x));
                    self.x = self.regs[ra] > self.regs[rb];
                }
                // shl
                0x10 => self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shl(1)),
                // shlc
                0x11 => {
                    self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shl(1) | u8::from(self.x));
                }
                // shlx
                0x12 => {
                    self.x = self.regs[rb].0 & 0x80 != 0;
                    self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shl(1));
                }
                // shlb
                0x13 => {
                    let tmp = self.regs[rb].0 & 0x80 != 0;
                    self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shl(1) | u8::from(self.x));
                    self.x = tmp;
                }
                // shr
                0x14 => self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shr(1)),
                // shrc
                0x15 => {
                    self.regs[ra] = Wrapping(
                        self.regs[rb].0.wrapping_shr(1) | u8::from(self.x).wrapping_shl(7),
                    );
                }
                // shrx
                0x16 => {
                    self.x = self.regs[ra].0 & 0x01 != 0;
                    self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shr(1));
                }
                // shrb
                0x17 => {
                    let tmp = self.regs[rb].0 & 0x01 != 0;
                    self.regs[ra] = Wrapping(
                        self.regs[rb].0.wrapping_shr(1) | u8::from(self.x).wrapping_shl(7),
                    );
                    self.x = tmp;
                }
                0x18 => {
                    if self.x {
                        self.regs[ra] = self.regs[rb];
                    }
                }
                0x19 => {
                    if !self.x {
                        self.regs[ra] = self.regs[rb];
                    }
                }
                // cad
                0x1a => {
                    if self.x {
                        self.regs[ra] = self.regs[ra] + self.regs[rb];
                    }
                }
                // csb
                0x1b => {
                    if self.x {
                        self.regs[ra] = self.regs[ra] - self.regs[rb];
                    }
                }
                0x1c => self.regs[ra] = -self.regs[rb],
                // swap nibbles
                0x1d => {
                    self.regs[ra] =
                        Wrapping(self.regs[ra].0.wrapping_shr(4) | self.regs[ra].0.wrapping_shl(4));
                }

                // push
                0x1e => {
                    self.regs[rb] -= Wrapping(1);
                    self.data[usize::from(self.regs[rb].0)] = self.regs[ra].0;
                }
                // pop
                0x1f => {
                    self.regs[ra] = Wrapping(self.data[usize::from(self.regs[rb].0)]);
                    self.regs[rb] += Wrapping(1);
                }
                _ => unreachable!(),
            }
        } else if inst.wrapping_shr(11) & 0b11 == 0b01 {
            match inst.wrapping_shr(8) & 0b111 {
                0b000 => self.regs[ra] &= imm,
                0b001 => self.regs[ra] |= imm,
                0b010 => self.regs[ra] ^= imm,
                0b011 => self.regs[ra] = imm,
                0b100 => self.x = self.regs[ra] & imm != Wrapping(0),
                0b101 => self.x = self.regs[ra] == imm,
                0b110 => self.x = self.regs[ra] >= imm,
                0b111 => self.x = (self.regs[ra].0 as i8) >= (imm.0 as i8),
                _ => unreachable!(),
            }
        } else {
            match inst.wrapping_shr(8) & 0x1f {
                // branch true flag
                0b10000 => {
                    if self.x {
                        self.pc = imm;
                    }
                }
                // branch false flag
                0b10001 => {
                    if !self.x {
                        self.pc = imm;
                    }
                }
                // jump to imm
                0b10010 => {
                    self.pc = imm;
                }
                // jump subroutine
                0b10011 => {
                    self.regs[ra] = self.pc;
                    self.pc = imm;
                }
                // branch true reg
                0b10100 => {
                    if self.regs[ra].0 != 0 {
                        self.pc = imm;
                    }
                }
                // branch flase reg
                0b10101 => {
                    if self.regs[ra].0 == 0 {
                        self.pc = imm;
                    }
                }
                // jump register
                0b10110 => {
                    self.pc = self.regs[ra] + imm;
                }
                // branch if true and decrement
                0b10111 => {
                    if self.regs[ra].0 != 0 {
                        self.regs[ra] -= Wrapping(1);
                        self.pc = imm;
                    }
                }
                // load direct
                0b11000 => {
                    self.regs[ra] = Wrapping(self.data[usize::from(imm.0)]);
                }
                // store direct
                0b11001 => {
                    self.data[usize::from(imm.0)] = self.regs[ra].0;
                }
                // port load
                0b11010 => match Port::try_from(usize::from(imm.0)) {
                    Ok(port) => {
                        self.regs[ra] = Wrapping(self.ports.read_port(port));
                    }
                    Err(_) => panic!("invalid port address: {}", imm.0),
                },
                // port store
                0b11011 => match Port::try_from(usize::from(imm.0)) {
                    Ok(port) => {
                        self.ports.write_port(port, self.regs[ra].0);
                    }
                    Err(_) => panic!("invalid port address: {}", imm.0),
                },
                // load offset
                0b11100 => {
                    self.regs[ra] = Wrapping(self.data[usize::from((self.regs[rb] + imm).0)]);
                }
                // store offset
                0b11101 => {
                    self.data[usize::from((self.regs[rb] + imm).0)] = self.regs[ra].0;
                }
                // add imm
                0b11110 => {
                    self.regs[ra] = self.regs[rb] + imm;
                }
                // halt
                0b11111 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }

        usize::from(usize::from(self.pc.0) < self.program.len())
    }
}

fn main() {
    run::<Photon>();
}
