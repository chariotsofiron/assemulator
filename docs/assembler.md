# Assemulator reference manual (WIP)


## Running the assembler

```shell
Usage: assemulator <PROCESSOR> <FILE> <COMMAND>

Commands:
  assemble  Assemble the program
  run       Run the program
  help      Print this message or the help of the given subcommand(s)

Arguments:
  <PROCESSOR>  The processor to use [possible values: risc16, bit-twiddler]
  <FILE>       Input file

Options:
  -h, --help  Print help information
```

## Example

```shell
$ bit-twiddler insertion_sort.asm run

Program: 40 bytes
Data: 5 bytes
-----------------
1
2
3
4
5
```

## General Syntax


`label:  (opcode|directive)? arg_list?   ; comment`

## Expressions

Supports decimal, hex, binary, and octal numbers.

## Directives

```
.i8     <expr>[, <expr>, ...]
        List of bytes, also supports .i16


.align  <expr>
        Insert as much zero bytes as required to reach an address
        where <n> low order bits are zero. For example
        align 2 would make an alignment to the next 32-bit boundary.
.fill   <value>, <n>
        fills memory with value, n times

.strz   "<string1>"[, "<string2>"...]
        array of null-terminated strings

.set    <symbol>,<expression>
        Create a new program symbol with the name <symbol> and assign
        to it the value of <expression>. If <symbol> is already assigned,
        it will contain a new value from now on.

.org    <exp>[,<fill>]
        Sets the address of the current code to exp


.macro  <name> [<argname1>[=<default>][,<argname2>...]]
        Defines a macro which can be referenced by <name>
.endm
        ends a macro definition

.f16    <float>
        parses a 16-bit floating point value

.i8.8   <decimal>[, <decimal>, ...]
        parses a fixed-point value with 8 fractional bits
        rounding to nearest true value.
        a.b => a+b should be a multiple of 8


```


## Macros

```
print_char:
        .macro  tmp_reg, char           ; declare a macro with two args
        mov     tmp_reg, char
        pst     tmp_reg, @char
        .endm

plot_point:
        .macro  x, y
        pst     x, xpos
        pst     y, ypos
        pst draw
        .endm
```
