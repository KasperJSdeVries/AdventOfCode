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
	size_t start_x;
	size_t start_y;

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
					start_x = x;
					start_y = y;
					break;
			}

			pipes[x][y] = p;
		}
	}

	size_t distances[line_count][line_len];
	memset(distances, 0, line_count * line_len * sizeof(size_t));

	struct pos prev_locs[2] = {{start_x, start_y}, {start_x, start_y}};

	struct pos current_locs[2] = {0};

	int idx = 0;
	if (start_x != 0 && (pipes[start_x - 1][start_y] == HORIZONTAL ||
	                     pipes[start_x - 1][start_y] == BEND_NE ||
	                     pipes[start_x - 1][start_y] == BEND_SE)) {
		current_locs[idx++] = (struct pos){start_x - 1, start_y};
	}
	if (start_x != line_len - 1 && (pipes[start_x + 1][start_y] == HORIZONTAL ||
	                                pipes[start_x + 1][start_y] == BEND_NW ||
	                                pipes[start_x + 1][start_y] == BEND_SW)) {
		current_locs[idx++] = (struct pos){start_x + 1, start_y};
	}
	if (start_y != 0 && (pipes[start_x][start_y - 1] == VERTICAL ||
	                     pipes[start_x][start_y - 1] == BEND_SE ||
	                     pipes[start_x][start_y - 1] == BEND_SW)) {
		current_locs[idx++] = (struct pos){start_x, start_y - 1};
	}
	if (start_y != line_count - 1 && (pipes[start_x][start_y + 1] == VERTICAL ||
	                                  pipes[start_x][start_y + 1] == BEND_NW ||
	                                  pipes[start_x][start_y + 1] == BEND_NE)) {
		current_locs[idx++] = (struct pos){start_x, start_y + 1};
	}

	size_t max_dist = 1;
	while (!pos_equal(current_locs[0], current_locs[1])) {
		for (int i = 0; i < 2; ++i) {
			struct pos tmp = current_locs[i];

			switch (pipes[current_locs[i].x][current_locs[i].y]) {
				case VERTICAL:
					if (prev_locs[i].y == current_locs[i].y - 1) {
						current_locs[i].y++;
					} else {
						current_locs[i].y--;
					}
					break;
				case HORIZONTAL:
					if (prev_locs[i].x == current_locs[i].x - 1) {
						current_locs[i].x++;
					} else {
						current_locs[i].x--;
					}
					break;
				case BEND_NE:
					if (prev_locs[i].x == current_locs[i].x + 1) {
						current_locs[i].y--;
					} else {
						current_locs[i].x++;
					}
					break;
				case BEND_NW:
					if (prev_locs[i].x == current_locs[i].x - 1) {
						current_locs[i].y--;
					} else {
						current_locs[i].x--;
					}
					break;
				case BEND_SE:
					if (prev_locs[i].x == current_locs[i].x + 1) {
						current_locs[i].y++;
					} else {
						current_locs[i].x++;
					}
					break;
				case BEND_SW:
					if (prev_locs[i].x == current_locs[i].x - 1) {
						current_locs[i].y++;
					} else {
						current_locs[i].x--;
					}
					break;
			}
			prev_locs[i] = tmp;
		}

		max_dist++;
	}

	printf("%zu", max_dist);

	return 0;
}
