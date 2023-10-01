mod assembler;
mod color;
mod cpu;
mod port;
mod screen;
mod util;
mod reg;

pub use cpu::{Cpu, Token};
pub use reg::Register;
pub use port::{State, Port};
pub use util::mask;
pub use util::parse_int;


use assembler::Assembler;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Instruction set simulator")]
struct Args {
    /// Action
    #[command(subcommand)]
    action: Action,

    /// Input file
    file: PathBuf,
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
fn run2<T: Cpu>(args: &Args) -> Result<(), String> {
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

pub fn run<T: Cpu>() {
    let args = Args::parse();
    run2::<T>(&args).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
}
