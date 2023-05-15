use crate::{assembler::Assembler, cpu::risc16::Risc16};
mod assembler;
mod color;
mod cpu;
mod port;
mod screen;
mod util;
mod word;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser)]
#[command(about = "Instruction set simulator")]
struct Args {
    /// The processor to use
    #[arg(value_enum)]
    processor: Processor,

    /// Action
    #[command(subcommand)]
    action: Action,

    /// Input file
    file: String,
}

#[derive(clap::ValueEnum, Clone)]
enum Processor {
    Risc16,
    BitTwiddler,
}

/// Actions that can be performed
#[derive(clap::Subcommand)]
enum Action {
    /// Assemble the program
    Assemble,
    /// Run the program
    Run,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let text = std::fs::read_to_string(&args.file).map_err(|x| x.to_string())?;

    let mut asm = Assembler::<Risc16>::new(&text);
    asm.assemble();
    Ok(())
}
