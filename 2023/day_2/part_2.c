#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Node {
	char *line;
	struct Node *next;
};

int main(int argc, char *argv[]) {
	if (argc != 2) {
		fprintf(stderr, "Please enter input file.\n");
		exit(EXIT_FAILURE);
	}

	FILE *fp = fopen(argv[1], "rb");
	if (fp == NULL) {
		fprintf(stderr, "Could not open file: %s\n", argv[1]);
		exit(EXIT_FAILURE);
	}

	fseek(fp, 0, SEEK_END);
	size_t file_size = ftell(fp);
	fseek(fp, 0, SEEK_SET);
	char buffer[file_size];
	fread(buffer, 1, file_size, fp);
	fclose(fp);

	struct Node *base = malloc(sizeof(struct Node));
	base->line = buffer;
	base->next = NULL;

	struct Node *temp = base;

	for (size_t i = 0; i < file_size; ++i) {
		if (buffer[i] == '\n') {
			buffer[i++] = '\0';
			if (i >= file_size) {
				break;
			}
			temp->next = malloc(sizeof(struct Node));
			temp = temp->next;
			temp->line = &buffer[i];
			temp->next = NULL;
		}
	}

	int result = 0;

	for (struct Node *iter = base; iter != NULL; iter = iter->next) {
		char *line_ptr = iter->line;
		line_ptr += 5;
		while (*line_ptr != ':') {
			line_ptr++;
		}

		int red_max = 0;
		int green_max = 0;
		int blue_max = 0;

		while (*line_ptr != '\0') {
			line_ptr += 2;
			int amount;
			sscanf(line_ptr, "%d", &amount);

			while (*line_ptr != ' ') {
				line_ptr++;
			}
			line_ptr++;

			switch (*line_ptr) {
				case 'r':
					line_ptr += 3;
					if (amount > red_max) {
						red_max = amount;
					}
					break;
				case 'g':
					line_ptr += 5;
					if (amount > green_max) {
						green_max = amount;
					}
					break;
				case 'b':
					line_ptr += 4;
					if (amount > blue_max) {
						blue_max = amount;
					}
					break;
			}
		}

		result += red_max * blue_max * green_max;
	}

	printf("%d\n", result);

	for (struct Node *iter = base; iter != NULL;) {
		struct Node *tmp = iter;
		iter = iter->next;
		free(tmp);
	}

	return 0;
}
