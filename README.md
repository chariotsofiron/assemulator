# Assemulator

Assemulator is a framework for designing, assembling, and emulating instruction sets.

By implementing the CPU trait, you get a fully-featured assembler and emulator.

```rust
fn new(pc: u64, program: Vec<u8>, data: Vec<u8>) -> Self;
fn parse(tokens: Vec<Token<Self::Opcode, Self::Reg>>, address: u64) -> Result<Vec<u8>, String>;
fn step(&mut self) -> usize;
```

## Assembler features

- Labels
- Immediate, relative, and direct addressing modes
- Constants (hex, binary, decimal, chars, strings)
- Directives (constants, set, fill, align)
- Expression evaluation: `1 + 2 * 3`, `1 << 2`, etc.
- Macros
- Conditional assembly
- Helpful error messages


## Emulator features

- Real-time user input for games via 6 buttons (arrow keys and ZX)
- Prompt user input from stdin
- Print char and int to stdout
- Generate random numbers
- Bitmap screen
- 200MHz+ emulation speed


## Features planned / in progress

- Fixed-point constants (i0.8, i5.3, i8.16, i32.32, etc.)
- Label scopes
- Include namespaces
- Step debugging


# Examples


```shell
$ cargo run --example risc16 cpu/risc16/programs/multiplication.asm run
Program: 28 bytes
Data: 0 bytes
Data: []
-----------------
1848
```
