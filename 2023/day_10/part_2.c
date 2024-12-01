#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
	FLOOR,
	VERTICAL,
	HORIZONTAL,
	BEND_NE,
	BEND_NW,
	BEND_SE,
	BEND_SW,
	START,
} Pipe;

struct pos {
	size_t x;
	size_t y;
};

bool pos_equal(struct pos a, struct pos b) {
	return a.x == b.x && a.y == b.y;
}

struct positions {
	struct pos *items;
	size_t length;
	size_t capacity;
};

struct positions positions_create(void) {
	return (struct positions){
	    .items = malloc(sizeof(struct pos) * 8),
	    .capacity = 8,
	    .length = 0,
	};
}

void positions_add(struct positions *arr, struct pos item) {
	if (arr->length + 1 >= arr->capacity) {
		arr->capacity *= 2;
		arr->items =
		    reallocarray(arr->items, arr->capacity, sizeof(struct pos));
	}
	arr->items[arr->length++] = item;
}

void positions_remove_at(struct positions *arr, size_t index) {
	memmove(
	    &arr->items[index], &arr->items[index + 1],
	    (arr->length - (index + 1)) * sizeof(struct pos)
	);
	arr->length--;
}

bool positions_find(struct positions arr, struct pos item, size_t *out_index) {
	for (size_t i = 0; i < arr.length; ++i) {
		if (pos_equal(arr.items[i], item)) {
			if (out_index != NULL) {
				*out_index = i;
			}
			return true;
		}
	}
	return false;
}

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

	size_t line_len = 0;
	while (buffer[line_len] != '\n') {
		line_len++;
	}

	size_t line_count = 0;
	for (size_t i = 0; i < file_size; ++i) {
		if (buffer[i] == '\n') {
			line_count++;
		}
	}

	Pipe pipes[line_len][line_count];
	struct pos start;

	for (size_t y = 0; y < line_count; ++y) {
		for (size_t x = 0; x < line_len; ++x) {
			Pipe p;
			char c = buffer[y * (line_len + 1) + x];

			switch (c) {
				case '.':
					p = FLOOR;
					break;
				case '|':
					p = VERTICAL;
					break;
				case '-':
					p = HORIZONTAL;
					break;
				case 'L':
					p = BEND_NE;
					break;
				case 'J':
					p = BEND_NW;
					break;
				case '7':
					p = BEND_SW;
					break;
				case 'F':
					p = BEND_SE;
					break;
				case 'S':
					p = START;
					start.x = x;
					start.y = y;
					break;
			}

			pipes[x][y] = p;
		}
	}

	struct pos prev_pos = start;

	struct pos current_pos = {0};

	Pipe start_type = 0;

	if (start.x != 0 && (pipes[start.x - 1][start.y] == HORIZONTAL ||
	                     pipes[start.x - 1][start.y] == BEND_NE ||
	                     pipes[start.x - 1][start.y] == BEND_SE)) {
		current_pos = (struct pos){start.x - 1, start.y};
		start_type = BEND_NW;
	}
	if (start.x != line_len - 1 && (pipes[start.x + 1][start.y] == HORIZONTAL ||
	                                pipes[start.x + 1][start.y] == BEND_NW ||
	                                pipes[start.x + 1][start.y] == BEND_SW)) {
		current_pos = (struct pos){start.x + 1, start.y};
		if (start_type == BEND_NW) {
			start_type = HORIZONTAL;
		} else {
			start_type = BEND_NE;
		}
	}
	if (start.y != line_count - 1 && (pipes[start.x][start.y + 1] == VERTICAL ||
	                                  pipes[start.x][start.y + 1] == BEND_NW ||
	                                  pipes[start.x][start.y + 1] == BEND_NE)) {
		current_pos = (struct pos){start.x, start.y + 1};
		if (start_type == BEND_NE) {
			start_type = BEND_SE;
		} else if (start_type == BEND_NW) {
			start_type = BEND_SW;
		} else {
			start_type = VERTICAL;
		}
	}
	if (start.y != 0 && (pipes[start.x][start.y - 1] == VERTICAL ||
	                     pipes[start.x][start.y - 1] == BEND_SE ||
	                     pipes[start.x][start.y - 1] == BEND_SW)) {
		current_pos = (struct pos){start.x, start.y - 1};
	}

	pipes[start.x][start.y] = start_type;

	struct positions border = positions_create();
	positions_add(&border, start);
	while (!pos_equal(current_pos, start)) {
		positions_add(&border, current_pos);
		struct pos tmp = current_pos;

		switch (pipes[current_pos.x][current_pos.y]) {
			case VERTICAL:
				if (prev_pos.y == current_pos.y - 1) {
					current_pos.y++;
				} else {
					current_pos.y--;
				}
				break;
			case HORIZONTAL:
				if (prev_pos.x == current_pos.x - 1) {
					current_pos.x++;
				} else {
					current_pos.x--;
				}
				break;
			case BEND_NE:
				if (prev_pos.x == current_pos.x + 1) {
					current_pos.y--;
				} else {
					current_pos.x++;
				}
				break;
			case BEND_NW:
				if (prev_pos.x == current_pos.x - 1) {
					current_pos.y--;
				} else {
					current_pos.x--;
				}
				break;
			case BEND_SE:
				if (prev_pos.x == current_pos.x + 1) {
					current_pos.y++;
				} else {
					current_pos.x++;
				}
				break;
			case BEND_SW:
				if (prev_pos.x == current_pos.x - 1) {
					current_pos.y++;
				} else {
					current_pos.x--;
				}
				break;
		}

		prev_pos = tmp;
	}

	bool areas[line_len][line_count];

	for (size_t y = 0; y < line_count; ++y) {
		bool inside = false;
		bool fromTop = false;
		for (size_t x = 0; x < line_len; ++x) {
			areas[x][y] = false;
			struct pos here = (struct pos){x, y};
			if (positions_find(border, here, NULL)) {
				Pipe p = pipes[x][y];

				switch (p) {
					case BEND_SE:
						fromTop = false;
						inside = !inside;
						break;
					case BEND_NE:
						fromTop = true;
						inside = !inside;
						break;
					case VERTICAL:
						inside = !inside;
						break;
					case BEND_NW:
						if (fromTop) {
							inside = !inside;
						}
						break;
					case BEND_SW:
						if (!fromTop) {
							inside = !inside;
						}
						break;
				}

				areas[x][y] = false;
			} else if (inside) {
				areas[x][y] = true;
			}
		}
	}

	size_t area = 0;

	for (size_t y = 0; y < line_count; ++y) {
		for (size_t x = 0; x < line_len; ++x) {
			if (areas[x][y]) {
				area++;
			}
		}
	}

	printf("%zu\n", area);

	return 0;
}
