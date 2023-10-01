use std::str::FromStr;

/// A token from the assembler
/// Register type is to be specified by the CPU.
/// This is better than an integer because we can have custom names for registers
#[derive(Debug)]
pub enum Token<U, T> {
    /// The opcode of the instruction
    Op(U),
    /// A register argument for an instruction
    Reg(T),
    /// An immediate argument for an instruction
    Imm(u64),
}

pub trait Cpu: Default {
    // type Opcode: for<'a> TryFrom<&'a str, Error = String> + std::fmt::Debug;
    // type Reg: for<'a> TryFrom<&'a str, Error = String> + std::fmt::Debug;
    type Opcode: FromStr;
    type Reg: FromStr;

    /// Creates a new state with the PC initialized.
    ///
    /// # Arguments
    ///
    /// * `pc` - The initial value of the program counter
    /// * `program` - The program instructions
    /// * `data` - The statically defined data
    fn new(pc: u64, program: Vec<u8>, data: Vec<u8>) -> Self;

    /// Parses a list of tokens into a list of bytes.
    /// Passes in the address that this instruction will be at
    ///
    /// Important: don't generate different length instruction
    /// based on size of immediate.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The tokens to parse
    /// * `address` - The address the instruction will be placed at
    fn parse(tokens: Vec<Token<Self::Opcode, Self::Reg>>, address: u64) -> Result<Vec<u8>, String>;

    /// Executes one instruction. Handles reading the instruction from memory, parsing
    /// it, and executing it. The function returns the number of cycles it took to execute.
    /// If zero is returned, the CPU is halted.
    fn step(&mut self) -> usize;
}
