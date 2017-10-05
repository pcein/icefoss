#include <string.h>
#include <stdio.h>

int main()
{
    char *c = strstr("hello", "mo");
    printf("%c\n", *c);
}
