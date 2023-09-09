#include <stdio.h>
#include <stdint.h>
#include <setjmp.h>

jmp_buf ugly;

int32_t multiply(int32_t a, int32_t b, void (*sheesh)()) {
    printf("Calling sheesh from C..\n");

    int8_t was_there = 0;

    setjmp(ugly);

    if (!was_there) {
        was_there = 1;
        sheesh();
    }

    return a * b;
}

void bad_jump_inside() {
    printf("About to jump!\n");
    //longjmp(ugly, "Kek");
}
