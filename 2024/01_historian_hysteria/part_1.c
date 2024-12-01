#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#define DA_INITIAL_CAPACITY 128

#define da_append(da, item)                                                                        \
    do {                                                                                           \
        if ((da)->count >= (da)->capacity) {                                                       \
            (da)->capacity = (da)->capacity == 0 ? DA_INITIAL_CAPACITY : (da)->capacity * 2;       \
            (da)->items = realloc((da)->items, (da)->capacity * sizeof((da)->items[0]));           \
            assert((da)->items && "Not enough RAM!");                                              \
        }                                                                                          \
        (da)->items[(da)->count++] = (item);                                                       \
    } while (0)
#define da_free(da) free((da).items)

typedef struct {
    int *items;
    size_t count;
    size_t capacity;
} List;

int compare(const void *p1, const void *p2) {
    int a = *(int *)p1;
    int b = *(int *)p2;

    return a - b;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <input file>\n", argv[0]);
        return 1;
    }

    FILE *fp = fopen(argv[1], "r");

    List left = {0}, right = {0};
    int l, r;
    char *line = NULL;
    size_t len;
    ssize_t nread;
    while ((nread = getline(&line, &len, fp)) != -1) {
        sscanf(line, "%d %d", &l, &r);
        da_append(&left, l);
        da_append(&right, r);
    }

    free(line);
    fclose(fp);

    qsort(left.items, left.count, sizeof(left.items[0]), compare);
    qsort(right.items, right.count, sizeof(right.items[0]), compare);

    int sum = 0;
    for (int i = 0; i < left.count; i++) {
        sum += abs(left.items[i] - right.items[i]);
    }

    printf("%d\n", sum);

    da_free(left);
    da_free(right);

    return 0;
}
