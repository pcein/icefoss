#include <stdio.h>

extern int c_add(int, int);

int main()
{
    printf("%d\n", c_add(10, 20));
    return 0;
}
