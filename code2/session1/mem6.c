#include <stdlib.h>

void fun(int *p)
{
    // use p
    free(p);
}
int main()
{
    int *p = malloc(10 * sizeof(int));
    fun(p);
    p[0] = 10;
}
