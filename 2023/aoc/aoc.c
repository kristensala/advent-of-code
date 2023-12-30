#include "aoc.h"

int read_file_lines(char *filename, char *out[]) {
    char *line = NULL;
    size_t len = 0;
    int line_count = 0;

    FILE *fp;
    fp = fopen(filename, "r");
    if (fp == NULL) {
        printf("Could not open file");
        exit(EXIT_FAILURE);
    }

    while ((getline(&line, &len, fp)) != -1) {
        line[strcspn(line, "\n")] = '\0'; // removes the end of line character

        out[line_count] = line;
        line_count++;
    }

    fclose(fp);
    return line_count;
}

int split_to_array(char *line, char *delim, char *result[]) {
    int count = 0;
    char *token = strtok(line, delim);

    // if delim does not exist in the line
    // then just return the line
    // in the result array
    if (token == NULL) {
        result[0] = line;
        return 1;
    }

    while (token != NULL) {
        result[count] = token;
        token = strtok(NULL, delim);
        count++;
    }

    return count;
}
