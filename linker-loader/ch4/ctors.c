/* WSL2上のgcc 9.4.0 では .ctor section が作成されなかった。
 * リンカスクリプトが原因かもしれない。
 */
#include <stdio.h>
#include <stdlib.h>

int count = 0;

void init1() {
  count++;
  printf("ctors test. (init1)\n");
}

void init2() {
  count++;
  count++;
  printf("ctors test. (init2)\n");
}

void (*fp1)(void) __attribute__((section(".ctors"))) = init1;
void (*fp2)(void) __attribute__((section(".ctors"))) = init2;

int main() {
  printf("%d\n", count);
  exit(0);
}
