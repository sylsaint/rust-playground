#include <stdlib.h>
#include <stdio.h>

// declare
extern void rust_capitalize(char *);

int main()
{
    char str[] = "hello world";
    rust_capitalize(str);
    printf("%s\n", str);
    return 0;
}