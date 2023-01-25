#include <stdio.h>
#include <stdlib.h>

extern int a;

int main() {
    a = 2;
    printf("%d\n", a);
    exit(0);
}
