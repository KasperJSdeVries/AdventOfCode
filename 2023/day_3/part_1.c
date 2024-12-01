#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

	size_t width = 0;
	size_t height = 0;
	for (size_t i = 0; i < file_size; ++i) {
		if (buffer[i] == '\n') {
			if (width == 0) {
				width = i;
			}
			height++;
		}
	}

	char schematic[height][width];
	for (size_t i = 0; i < height; ++i) {
		memcpy(&schematic[i], buffer + (width + 1) * i, width);
	}

	int result = 0;
	for (size_t y = 0; y < height; ++y) {

		size_t number_begin_column;
		bool in_number = false;

		for (size_t x = 0; x < width; ++x) {
			if (in_number) {
				if (!isdigit(schematic[y][x])) {
					in_number = false;
					bool has_adjacent_symbol = false;
					for (size_t k = number_begin_column; k < x; ++k) {
						for (int i = y == 0 ? 0 : -1;
						     i < (y < height - 1 ? 2 : 1); ++i) {
							for (int j = k == 0 ? 0 : -1; j < 2; ++j) {
								char c = schematic[y + i][k + j];
								if (!isdigit(c) && c != '.') {
									has_adjacent_symbol = true;
								}
							}
						}
					}
					if (has_adjacent_symbol) {
						char *str = strndup(
						    &schematic[y][number_begin_column],
						    x - number_begin_column
						);
						int number = atoi(str);
						free(str);

						// printf("%d\n", number);

						result += number;
					}
				}
			} else if (isdigit(schematic[y][x])) {
				in_number = true;
				number_begin_column = x;
			}
		}

		if (in_number) {
			in_number = false;
			bool has_adjacent_symbol = false;
			for (size_t k = number_begin_column; k < width; ++k) {
				for (int i = y == 0 ? 0 : -1; i < (y < height - 1 ? 2 : 1);
				     ++i) {
					for (int j = k == 0 ? 0 : -1; j < 1; ++j) {
						char c = schematic[y + i][k + j];
						if (!isdigit(c) && c != '.') {
							has_adjacent_symbol = true;
						}
					}
				}
			}
			if (has_adjacent_symbol) {
				char *str = strndup(
				    &schematic[y][number_begin_column],
				    width - number_begin_column
				);
				int number = atoi(str);
				free(str);

				// printf("%d at %zu, %zu\n", number, number_begin_column, y);

				result += number;
			}
		}
	}

	printf("%d\n", result);

	return 0;
}
