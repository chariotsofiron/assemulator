
        ; zero-argument no operation
nop:    .macro
        add r0, r0, r0
        .endm

not:    .macro dest, src
        nand $dest, $src, $src
        .endm

        ; print the value of a register as an unsigned integer
        ; using memory-mapped I/O
print:  .macro reg
        st $reg, r0, ticker
        .endm


        ; load a 16-bit immediate into a register
movi:   .macro reg, imm
        lui $reg, $imm >> 10
        addi $reg, r0, $imm & 0x2f
        .endm

        ; absolute value
        ; this uses the "not" macro. Should there be a symbol
        ; to distinguish between macros and opcodes?
abs:    .macro reg, temp
        lui $temp, 0x8000
        nand $temp, $reg, $temp
        not $temp, $temp
        beq $temp, r0, +
        sub $reg, r0, $reg
@:


jump:   .macro label
        beq r0, r0, $label
        .endm


print_string:



print_string:   .macro string
        .scope
        str:    .strz $string           ; declare string statically
                movi r1, str            ; load address of string

        loop:   ld r2, r1, 0            ; load first character
                beq r2, r0, end
                st r2, r0, char         ; print character
                addi r1, r1, 1          ; increment address
                jump loop
        end:
        .endscope



        
        print_string "Hello, world!"



; recursive


