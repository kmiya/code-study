#include <stdio.h>
#include <stdlib.h>

void __attribute__((constructor)) init() { printf("init()\n"); }

void __attribute__((destructor)) fini() { printf("fini()\n"); }

int main() {
  printf("main()\n");
  exit(0);
}
