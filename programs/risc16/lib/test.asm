
        .include "macros.asm"
        .zero 32


text:   .i16 'a', 'b', '\n', 0

main:   ld r1, r2, text
        ; st r1, r0, ticker 
        beq r1, r0, end2
        add r2, r2, 1
        st r1, r0, char
        jmp main

end: