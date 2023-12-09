#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_RED 12
#define MAX_GREEN 13
#define MAX_BLUE 14

typedef struct {
    int blue_cubes;
    int green_cubes;
    int red_cubes;
} set_t;

typedef struct {
    int id;
    set_t sets[10];
} game_t;

set_t init_set(void) {
    set_t result;
    result.green_cubes = 0;
    result.red_cubes = 0;
    result.blue_cubes = 0;
    return result;
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

    return count; // return len of the array
}

game_t parse_game(char *line) {
    game_t game;

    char *game_parts[1000];
    split_to_array(line, ":", game_parts);

    char *game_id_part[5];
    split_to_array(game_parts[0], " ", game_id_part);

    game.id =  atoi(game_id_part[1]);

    char *game_sets[10];
    int game_sets_len = split_to_array(game_parts[1], ";", game_sets);

    for (int i = 0; i < game_sets_len; ++i) {
        set_t set = init_set();

        char *set_array[3];
        int set_array_len = split_to_array(game_sets[i], ",", set_array);

        for (int j = 0; j < set_array_len; ++j) {
            char *set_elements[3];
            split_to_array(set_array[j], " ", set_elements);

            if (strcmp(set_elements[1], "blue") == 0) {
                set.blue_cubes = atoi(set_elements[0]);
            } else if (strcmp(set_elements[1], "red") == 0) {
                set.red_cubes = atoi(set_elements[0]);
            } else if (strcmp(set_elements[1], "green") == 0) {
                set.green_cubes = atoi(set_elements[0]);
            } else {
                continue;
            }
        }

        game.sets[i] = set;
    }

    return game;
}


int part_two(char *line) {
    parse_game(line);

    return 0;
}

int main() {
    int part_two_result = 0;

    char *line = NULL;
    size_t len = 0;

    FILE *fp;
    fp = fopen("day2-1.test", "r");
    if (fp == NULL) {
        printf("Could not open file");
        exit(EXIT_FAILURE);
    }

    while ((getline(&line, &len, fp)) != -1) {
        line[strcspn(line, "\n")] = '\0'; // removes the end of line character

        part_two_result += part_two(line);
    }

    fclose(fp);

    return 0;
}