#include <my_header.h>
#include <stdio.h>

int main() {
    uint16_t a = 8;
    uint16_t b = 2;

    uint16_t c = add(&a, &b);

    printf("Num: %d\n", c );

}