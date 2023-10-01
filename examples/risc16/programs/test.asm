        .include "macros.asm"

bar:    .set -1
        movi r1, bar

        .if bar
        print r1
        .endif
        