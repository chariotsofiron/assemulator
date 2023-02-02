
KEY_UP:         .set 1
KEY_DOWN:       .set 2
KEY_LEFT:       .set 4
KEY_RIGHT:      .set 8


; get user input

        ; init
        mov a, 15       ; x pos
        mov b, 15       ; y pos
        mov c, 1        ; dx
        mov d, -1       ; dy

loop:
        ; read buttons
        pld e, buttons

        ; handle controls
        tst e, KEY_UP
        cad b, d
        tst e, KEY_DOWN
        cad b, c
        tst e, KEY_LEFT
        cad a, d
        tst e, KEY_RIGHT
        cad a, c

        pst a, xpos
        pst b, ypos
        pst flip


; print apple

        pld a, RNG
        and a, 0b11111
        pst a, X
        
        pld a, RNG
        and a, 0b11111
        pst a, Y

        mov a, color
        pst a, COLOR