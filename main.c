#include <stdio.h>

#include "imgmath.h"

int main(void)
{
  // Create a string with value asd
  char *str = "asd";

  char *output = blue_light_filter(4000.0, "a.png");
  printf("%s\n", output);
  rust_free_string(output);

  return 0;
}
