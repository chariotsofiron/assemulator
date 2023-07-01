use crate::{cpu::risc16::Risc16};
/// Assembler
mod assembler;
/// Colors for screen
mod color;
/// CPUs
mod cpu;
/// I/O ports
mod port;
/// The screen
mod screen;
/// Utility functions
mod util;
use clap::Parser;
use cpu::Cpu;

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

/// The supported CPUs
#[derive(clap::ValueEnum, Clone)]
enum Processor {
    /// Risc16 CPU
    Risc16,
}

/// Actions that can be performed
#[derive(clap::Subcommand)]
enum Action {
    /// Assemble the program
    Assemble,
    /// Run the program
    Run,
}

/// Run the program
fn run<T: Cpu>(args: &Args) -> Result<(), String> {
    // let mut asm = Assembler::<T>::new(&args.file);
    // asm.assemble()?;

    // println!("Program: {} bytes", asm.program.len());
    // println!("Data: {} bytes", asm.data.len());
    // println!("-----------------");

    // match args.action {
    //     Action::Assemble => {
    //         println!("Data: {:#04x?}", asm.data);
    //         println!("Program: {:#04x?}", asm.program);
    //     }
    //     Action::Run => {
    //         let pc = asm.symbols.get("main").copied().unwrap_or_default();
    //         let mut state = T::new(pc, asm.program, asm.data);
    //         while state.step() != 0 {}
    //         // println!("State: {:#?}", state);
    //     }
    // }
    Ok(())
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    // let text = std::fs::read_to_string(&args.file).map_err(|x| x.to_string())?;

    match args.processor {
        Processor::Risc16 => run::<Risc16>(&args)?,
    }

    Ok(())
}
