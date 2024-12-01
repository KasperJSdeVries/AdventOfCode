#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	char value[4];
	size_t left_index;
	size_t right_index;
} Node;

enum compare_result {
	GREATER,
	EQUAL,
	LESS
};

enum compare_result compare(char lhs[3], char rhs[3]) {
	for (int i = 2; i >= 0; i--) {
		if (lhs[i] < rhs[i]) {
			return LESS;
		}
		if (lhs[i] > rhs[i]) {
			return GREATER;
		}
	}

	return EQUAL;
}

void merge(char arr[][17], size_t l, size_t m, size_t r) {
	size_t n1 = m - l + 1;
	size_t n2 = r - m;

	char L[n1][17], R[n2][17];

	for (size_t i = 0; i < n1; ++i) {
		memcpy(L[i], arr[l + i], 17);
	}
	for (size_t i = 0; i < n2; ++i) {
		memcpy(R[i], arr[m + 1 + i], 17);
	}

	size_t i = 0;
	size_t j = 0;
	size_t k = l;

	while (i < n1 && j < n2) {
		if (compare(L[i], R[j]) != GREATER) {
			memcpy(arr[k], L[i], 17);
			i++;
		} else {
			memcpy(arr[k], R[j], 17);
			j++;
		}
		k++;
	}

	while (i < n1) {
		memcpy(arr[k], L[i], 17);
		i++;
		k++;
	}

	while (j < n2) {
		memcpy(arr[k], R[j], 17);
		j++;
		k++;
	}
}

void merge_sort(char arr[][17], size_t l, size_t r) {
	if (l < r) {
		size_t m = (l + r) / 2;

		merge_sort(arr, l, m);
		merge_sort(arr, m + 1, r);

		merge(arr, l, m, r);
	}
}

size_t binary_search(char arr[][17], size_t item_count, char item[3]) {
	size_t L = 0;
	size_t R = item_count - 1;
	while (L <= R) {
		size_t m = (L + R) / 2;
		enum compare_result cr = compare(arr[m], item);
		if (cr == LESS) {
			L = m + 1;
		} else if (cr == GREATER) {
			R = m - 1;
		} else {
			return m;
		}
	}
	return item_count;
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

	char *bp = buffer;
	char *instructions = buffer;

	while (*bp != '\n') {
		bp++;
	}

	*bp = '\0';
	bp += 2;

	char *ncp = bp;
	size_t node_count = 0;
	while (*ncp != '\0') {
		if (*ncp == '\n') {
			node_count++;
		}
		ncp++;
	}

	char node_strings[node_count][17];

	for (size_t i = 0; i < node_count; ++i) {
		memcpy(node_strings[i], bp, 16);
		node_strings[i][16] = '\0';
		bp += 17;
	}

	merge_sort(node_strings, 0, node_count - 1);

	Node nodes[node_count];

	for (size_t i = 0; i < node_count; ++i) {
		memcpy(nodes[i].value, node_strings[i], 3);
		nodes[i].value[3] = '\0';

		char tmp[3];
		memcpy(tmp, node_strings[i] + 7, 3);
		nodes[i].left_index = binary_search(node_strings, node_count, tmp);
		memcpy(tmp, node_strings[i] + 12, 3);
		nodes[i].right_index = binary_search(node_strings, node_count, tmp);
	}

	size_t proc_node_count = 0;

	for (size_t i = 0; i < node_count; ++i) {
		if (nodes[i].value[2] == 'A') {
			proc_node_count++;
		} else {
			break;
		}
	}

	Node proc_nodes[proc_node_count];
	memcpy(proc_nodes, nodes, sizeof(Node) * proc_node_count);
	size_t iptr = 0;
	size_t steps = 0;
	size_t instructions_len = strlen(instructions);

	for (;;) {
		bool all_reached = true;
		for (size_t i = 0; i < proc_node_count; ++i) {
			if (proc_nodes[i].value[2] != 'Z') {
				all_reached = false;
			} else {
				printf("%zu: ", steps);
				for (size_t i = 0; i < proc_node_count; ++i) {
					printf("%s ", proc_nodes[i].value);
				}
				printf("\n");
			}
		}
		if (all_reached) {
			break;
		}

		if (iptr >= instructions_len) {
			iptr = 0;
		}

		// if (steps % 1000000000 == 0) {
		//	printf("%zu: ", steps);
		//	for (size_t i = 0; i < proc_node_count; ++i) {
		//		printf("%s ", proc_nodes[i].value);
		//	}
		//	printf("\n");
		// }

		for (size_t i = 0; i < proc_node_count; ++i) {
			if (instructions[iptr] == 'L') {
				proc_nodes[i] = nodes[proc_nodes[i].left_index];
			} else {
				proc_nodes[i] = nodes[proc_nodes[i].right_index];
			}
		}

		iptr++;
		steps++;
	}

	printf("%zu\n", steps);

	return 0;
}
