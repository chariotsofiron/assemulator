//! Opcodes for the CPU
use enum_map::Enum;
use strum_macros::EnumString;

/// Assembler mnemonics
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum Mnemonic {
    Add,
    Cmp,
    Sub,
    Sbc,
    Adc,
    And,
    Ior,
    Xor,
    Mov,
    Shl,
    Shr,
    Rol,
    Ror,
    Inc,
    Dec,
    Jmp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Enum, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Register {
    A,
    B,
    C,
    RamK,
    X,
    RamX,
    Y,
    RamY,
}

impl From<Register> for u16 {
    fn from(reg: Register) -> Self {
        match reg {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
            Register::RamK => 3,
            Register::X => 4,
            Register::RamX => 5,
            Register::Y => 6,
            Register::RamY => 7,
        }
    }
}

impl From<u16> for Register {
    fn from(value: u16) -> Self {
        match value & 0b111 {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::RamK,
            4 => Self::X,
            5 => Self::RamX,
            6 => Self::Y,
            7 => Self::RamY,
            _ => unreachable!(),
        }
    }
}

impl Register {
    /// Places the register in the A field of an instruction.
    pub fn to_a(self) -> u16 {
        u16::from(self).wrapping_shl(13)
    }

    /// Places the register in the B field of the instruction.
    pub fn to_b(self) -> u16 {
        u16::from(self).wrapping_shl(5)
    }
}
