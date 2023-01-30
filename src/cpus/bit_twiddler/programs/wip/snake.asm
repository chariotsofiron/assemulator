

; get user input

        ; init
        mov a, 15       ; x pos
        mov b, 15       ; y pos
        mov c, 1        ; dx
        mov d, -1       ; dy

        mov g, 1
        mov h, -1       ; just for copying

        ; read buttons
        pld e, buttonsp

        ; handle controls
        tst e, 1        ; up
        cad b, d
        tst e, 2        ; down
        cad b, c
        tst e, 4        ; left
        cad a, d
        tst e, 8        ; right
        cad a, c


        
        eq a, 'w'
        bt up
        eq a, 's'
        bt down
        eq a, 'a'
        bt left
        eq a, 'd'
        bt right


; print apple

        pld a, RNG
        and a, 0b11111
        pst a, X
        
        pld a, RNG
        and a, 0b11111
        pst a, Y

        mov a, color
        pst a, COLOR