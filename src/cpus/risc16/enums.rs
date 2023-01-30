use enum_map::Enum;
use strum_macros::EnumString;

/// Registers
#[derive(Debug, Copy, Clone, PartialEq, Eq, Enum, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Mnemonic {
    Add,
    Adi,
    Nand,
    Lui,
    St,
    Ld,
    Beq,
    Jalr,
}


/// Registers
#[derive(Debug, Copy, Clone, PartialEq, Eq, Enum, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

impl From<Register> for u16 {
    fn from(reg: Register) -> Self {
        match reg {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
        }
    }
}

impl From<u16> for Register {
    fn from(reg: u16) -> Self {
        match reg {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            _ => panic!("Invalid register"),
        }
    }
}

impl Register {
    pub fn to_a(self) -> u16 {
        u16::from(self).wrapping_shl(10)
    }
    pub fn to_b(self) -> u16 {
        u16::from(self).wrapping_shl(7)
    }
    pub fn to_c(self) -> u16 {
        u16::from(self)
    }
}
