pub mod risc16;
mod reg;

/// A token from the assembler
/// Register type is to be specified by the CPU.
/// This is better than an integer because we can have custom names for registers
#[derive(Debug)]
pub enum Token<'a, T> {
    /// The opcode of the instruction
    Op(&'a str),
    /// A register argument for an instruction
    Reg(T),
    /// An immediate argument for an instruction
    Imm(u64),
}

pub trait Cpu: Default {
    type Reg: for<'a> TryFrom<&'a str> + std::fmt::Debug;

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
    /// # Arguments
    ///
    /// * `tokens` - The tokens to parse
    /// * `address` - The address that this instruction will be at
    fn parse_tokens(tokens: Vec<Token<Self::Reg>>, address: u64) -> Result<Vec<u8>, String>;

    /// Executes one instruction. Handles reading the instruction from memory, parsing
    /// it, and executing it. The function returns the number of cycles it took to execute.
    /// If zero is returned, the CPU is halted.
    fn step(&mut self) -> usize;
}
