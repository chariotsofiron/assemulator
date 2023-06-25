use crate::{assembler::Assembler, cpu::risc16::Risc16};
mod assembler;
mod color;
mod cpu;
mod port;
mod screen;
mod util;
mod word;
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

#[derive(clap::ValueEnum, Clone)]
enum Processor {
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
fn run<T: Cpu>(args: &Args, text: &str) {
    let mut asm = Assembler::<T>::new(text);
    asm.assemble().unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    });

    println!("Program: {} bytes", asm.program.len());
    println!("Data: {} bytes", asm.data.len());
    println!("-----------------");

    match args.action {
        Action::Assemble => {
            println!("Data: {:#04x?}", asm.data);
            println!("Program: {:#04x?}", asm.program);
        }
        Action::Run => {
            let pc = asm.symbols.get("main").copied().unwrap_or_default();
            let mut state = T::new(pc, asm.program, asm.data);
            while state.step() != 0 {}
            // println!("State: {:#?}", state);
        }
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let text = std::fs::read_to_string(&args.file).map_err(|x| x.to_string())?;

    match args.processor {
        Processor::Risc16 => run::<Risc16>(&args, &text),
    }

    Ok(())
}
