        add r2, r0, 5
        add r3, r0, 3
        add r4, r0, 1           ; mask = 1
        beq r0, r0, mult

; 8x8=8 bit shifting multiplication
; r1: product
; r2: multiplicand
; r3: multiplier
; r4: mask

doadd:  add r1, r1, r2          ; product += multiplicand
loop:   add r2, r2, r2          ; multiplicand <<= 1
        add r4, r4, r4          ; mask <<= 1
        beq r4, r0, end
mult:   nand r5, r3, r4
        nand r5, r5, r5         ; r5 = mask & multiplier)
        beq r5, r4, doadd
        beq r0, r0, loop
        
end:    st r1, r0, ticker
