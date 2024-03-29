# Assemulator reference manual (WIP)


## Running the assembler

```shell
Instruction set simulator

Usage: <CPU> <FILE> <COMMAND>

Commands:
  assemble  Assemble the program
  run       Run the program
  help      Print this message or the help of the given subcommand(s)

Arguments:
  <FILE>  Input file

Options:
  -h, --help  Print help
```

## General Syntax


`(label:)?  (opcode arg_list?)?  (; comment)?`

## Expressions

Supports decimal, hex, binary, and octal numbers.

## Directives

```
.i8     <expr>[, <expr>, ...]
        List of bytes, also supports .i16, etc.

.f16    <float>[, <float>, ...]
        parses a 16-bit floating point value

.i8.8   <decimal>[, <decimal>, ...]
        parses a fixed-point value with 8 fractional bits
        rounding to nearest true value.
        ia.b => a+b should be a multiple of 8

.zero   <count>
        fills memory with n zeros

.strz   "<string1>"[, "<string2>"...]
        array of null-terminated strings

.set    <expression>
        Set the value of the label to <expression>
        These need to be forward declared

.align  <expr>
        Insert as much zero bytes as required to reach an address
        where <n> low order bits are zero. For example
        align 2 would make an alignment to the next 32-bit boundary.

.org    <exp>[,<fill>]
        Sets the address of the current code to exp


.macro  <name> [<argname1>[=<default>][,<argname2>...]]
        Defines a macro which can be referenced by <name>
.endm
        ends a macro definition

.if     <expr>
        If the expression is true, the following code is assembled
        until .endif is reached. .else can be used to specify code
        to assemble if the expression is false.

.include <file>
        Includes the contents of <file> at the current position
        in the assembly file. If label is present, all labels in
        <file> are prefixed with it


.fill   
        Not implemented - what data size to use?
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

macros that use .if?





# Feature ideas

## Anonymous labels

- sometimes we don't want to give a label an explicit name


## Expressions



## Macros



## Label scopes

- Could replace anonymous labels


## Conditional assembly

- e.g. want to move an immediate and want to generate different assembly depending on the value of the immediate

```
    .if <expr> > <expr>
    ...code...
    .endif
```



