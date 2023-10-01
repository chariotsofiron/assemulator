; Speed test, how many cycles to decrement 0x0f_ff_ff_ff
; Observed ~248MHz on Ryzen 5 5600X
        mov a, 0xff
        mov b, 0xff
        mov c, 0xff
        mov d, 0x0f

loop:   add a, -1
        bt a, loop
        add b, -1
        bt b, loop
        add c, -1
        bt c, loop
        add d, -1
        bt d, loop
