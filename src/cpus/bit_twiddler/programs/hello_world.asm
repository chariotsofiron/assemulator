; Prints a null-terminated string
; arg a: pointer to string
; clobbers b
loop:   pst b, char     ; print char
str.print:
        pop b, a        ; load char
        bt b, loop      ; loop while char != 0
        ret

hello:  .strz "Hello, World!\n"

main:   mov a, hello
        jsr str.print
