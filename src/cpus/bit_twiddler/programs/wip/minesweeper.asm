



board:  .zero 64
n_mines:    .const 10

        ; initialize mines
        mov c, n_mines
@:      pld a, RAND
        and a, 0b00111111

        mov b, board
        st b, 

        btd c, -



