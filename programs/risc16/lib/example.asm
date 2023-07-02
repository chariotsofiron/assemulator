
        .include "std.asm"

        nop


blah:   .set foo.bar

foo.bar:
        add r2, r3, blah

hello:  .strz "hello"

arr:    .i8 1, 2, 3