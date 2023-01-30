; ; Nth fibonacci
; 
; ; recursive fibonacci
; 
; fib2:   
;         ld a, g, -1     ; load n
;         eq a, 0
;         bt end
;         eq a, 1
;         bt end
; 
;         adi b, a, -1
;         psh b, g
;         jsr fib2
; 
; end:    ret


; Fibonacci
; prints fib numbers up to 233
; 0,1,1,2,3,5,...
; tmp a: num1
; tmp b: num2
fib:    mov a, 1
        mov b, 0
        mov c, 6
@:      add a, b
        pst b, @ticker
        add b, a
        pst a, @ticker
        btd c, -
        ret


main:   jsr fib