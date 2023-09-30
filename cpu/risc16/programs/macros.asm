movi:   .macro dest, imm
        add $dest, r0, $imm
        .endm

and:    .macro dest, src1, src2
        nand $dest, $src1, $src2
        not $dest, $dest
        .endm

jmp:    .macro dest
        beq r0, r0, $dest
        .endm

not:    .macro dest, src
        nand $dest, $src, $src
        .endm

sub:    .macro dest, src1, src2
        not $dest, $src2                ; invert src2
        add $dest, $dest, 1             ; add 1 to src2
        add $dest, $dest, $src1         ; add src1 to src2
        .endm

print:  .macro src
        st $src, r0, ticker
        .endm