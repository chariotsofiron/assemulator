#[derive(Debug)]
pub enum Opcode {
    Add,
    Nand,
    Lui,
    Ld,
    St,
    Beq,
    Jalr,
}

impl TryFrom<&str> for Opcode {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" => Ok(Self::Add),
            "nand" => Ok(Self::Nand),
            "lui" => Ok(Self::Lui),
            "ld" => Ok(Self::Ld),
            "st" => Ok(Self::St),
            "beq" => Ok(Self::Beq),
            "jalr" => Ok(Self::Jalr),
            _ => Err("Invalid opcode".to_owned()),
        }
    }
}
