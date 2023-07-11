use std::path::PathBuf;

use crate::{assembler::assembler::Assembler, cpu::risc16::Risc16, util::input};
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
    file: PathBuf,
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
#[allow(clippy::print_stdout)]
fn run<T: Cpu>(args: &Args) -> Result<(), String> {
    let asm = Assembler::<T>::assemble(&args.file)?;

    println!("Program: {} bytes", asm.program.len());
    println!("Data: {} bytes", asm.data.len());
    println!("Data: {:?}", asm.data);
    println!("-----------------");

    match args.action {
        Action::Assemble => {
            println!("Symbols: {:#?}", asm.symbols);
            println!("Data: {:#04x?}", asm.data);
            println!("Program: {:#04x?}", asm.program);
            // for line in asm
            //     .program
            //     .chunks_exact(2)
            //     .map(|x| u16::from_be_bytes([x[0], x[1]]))
            // {
            //     println!("{:#018b}", line);
            // }
        }
        Action::Run => {
            let pc = asm.symbols.get("main").copied().unwrap_or_default();
            let mut state = T::new(pc, asm.program, asm.data);
            while state.step() != 0 {
                // input("> ");
            }
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    match args.processor {
        Processor::Risc16 => run::<Risc16>(&args),
    }
    .unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
}
