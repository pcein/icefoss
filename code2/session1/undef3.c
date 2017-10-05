#include <limits.h>
#include <stdio.h>

main()
{
    int i = INT_MAX;

    if (i > i + 1) {
        printf("hello\n");
    }
}
