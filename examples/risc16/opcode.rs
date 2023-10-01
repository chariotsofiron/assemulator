use strum_macros::EnumString;

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Opcode {
    Add,
    Nand,
    Lui,
    Ld,
    St,
    Beq,
    Jalr,
}

