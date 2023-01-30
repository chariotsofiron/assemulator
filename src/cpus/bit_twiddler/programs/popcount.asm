; Popcount (Brian Kernighan’s algorithm)
; Run time proportional to the number of bits set
; out a: count
; arg b: input
; c: temp
@:      and b, c        ; b = b & (b - 1)
        add a, 1        ; a += 1
popcnt: add c, b, -1    ; c = b - 1
        bt b, -
        ret

main:   mov b, 0b101111
        jsr popcnt
        pst a, ticker
