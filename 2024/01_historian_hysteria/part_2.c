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

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <input file>\n", argv[0]);
        return 1;
    }

    FILE *fp = fopen(argv[1], "r");

    List left = {0}, right_freq = {0};
    int l, r;
    char *line = NULL;
    size_t len;
    ssize_t nread;
    while ((nread = getline(&line, &len, fp)) != -1) {
        sscanf(line, "%d %d", &l, &r);
		da_append(&left, l);

        for (int i = right_freq.count; i <= r; i++) {
            da_append(&right_freq, 0);
        }
        right_freq.items[r]++;
    }

    free(line);
    fclose(fp);

	int score = 0;
	for (int i=0 ; i < left.count; i++) {
		score += left.items[i] * right_freq.items[left.items[i]];
	}
	printf("%d\n", score);

    da_free(left);
    da_free(right_freq);

    return 0;
}
