#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef AOC_AOC_H
#define AOC_AOC_H

int read_file_lines(char *filename, char *out[]);
int split_to_array(char *line, char *delim, char *result[]);

#endif //AOC_AOC_H
