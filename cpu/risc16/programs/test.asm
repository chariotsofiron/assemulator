        .include "macros.asm"

heyo:   .macro x
        addi r1, r1, $x & 63
        .if x >> 6 != 1
        .endm

bar:    .set -1

        movi r1, bar

        .if bar
        print r1
        .endif
        