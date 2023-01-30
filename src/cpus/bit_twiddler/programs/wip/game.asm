        ; initialize position
        mov a, 15       ; x pos
        mov b, 15       ; y pos
        mov c, 1
        mov d, -1

loop:   pld e, buttons  ; get user input
        
        ; handle controls
        tst e, 1        ; up
        cad b, d
        tst e, 2        ; down
        cad b, c
        tst e, 4        ; left
        cad a, d
        tst e, 8        ; right
        cad a, c

        ; draw pixel
        pst a, xpos
        pst b, ypos
        pst a, flip
        jmp loop