#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

char s1[] = "This is array.";
char* s2 = "This is pointer.";

int main() {
  printf(" s1 = 0x%12lx\n", (uintptr_t)s1);
  printf("&s1 = 0x%12lx\n", (uintptr_t)&s1);
  printf(" s2 = 0x%12lx\n", (uintptr_t)s2);
  printf("&s2 = 0x%12lx\n", (uintptr_t)&s2);
  printf("sizeof(s1) = %ld\n", sizeof(s1));
  printf("sizeof(s2) = %ld\n", sizeof(s2));
  exit(0);
}
