#include <stdlib.h>

void fun()
{
    int *p = malloc(10 * sizeof(int));
    /* use the block */
    /* forget to free */
}
