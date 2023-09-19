mod cpu;
mod opcode;

use assemulator::run;
use cpu::Risc16;

fn main() {
    run::<Risc16>();
}
