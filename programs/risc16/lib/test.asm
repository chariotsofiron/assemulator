
        .include "macros.asm"
        .zero 32


text:   .i16 'a'

main:   ld r1, r2, text
        beq r1, r0, end
        add r2, r2, 1
        st r1, r0, char
        jmp main

end:
