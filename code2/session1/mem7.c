#include <stdlib.h>

void fun(int *p)
{
    // do some stuff with p

    free(p);
}
int main()
{
    int *p = malloc(10 * sizeof(int));
    fun(p);
    free(p); 
}
