#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

bool check_level(const char *line) {
    int prev;
    bool increasing;

    const char *c = line;
    for (int i = 0;; i++) {
        int report = atoi(c);
        if (i == 0) {
            goto next;
        }

        // can't have two of the same reports after eachother
        if (report == prev) {
            return false;
        }

        if (i == 1) {
            increasing = prev - report > 0;
        }

        int diff = prev - report;
        if (increasing && (diff > 3 || report > prev) ||
            !increasing && (diff < -3 || report < prev)) {
            return false;
        }

    next:
        c = strchr(c, ' ');
        if (c == NULL) {
            break;
        }
        c++;
        prev = report;
    }

    return true;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <input file>\n", argv[0]);
        return 1;
    }

    FILE *fp = fopen(argv[1], "r");

    int safe_reports = 0;

    char *line = NULL;
    size_t len;
    ssize_t nread;
    while ((nread = getline(&line, &len, fp)) != -1) {
        char *c = line;

        safe_reports += check_level(line);
    }

    printf("%d\n", safe_reports);

    free(line);
    fclose(fp);

    return 0;
}
