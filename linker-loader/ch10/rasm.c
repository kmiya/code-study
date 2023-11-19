extern int extvalue;
extern int extbssvalue;
int extfunc();

static int dummy_value = 3;
static int dummy_bssvalue;
static int value = 1; /* .data section */
static int bssvalue;  /* .bss section */
char *string;

static void dummy_func() {}

static int func() { return (extvalue + extbssvalue + value + bssvalue); }

int main() {
  extfunc();
  func();
  string = "sample_string";
  return (0);
}
