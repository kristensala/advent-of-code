#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "aoc.h"

#define MAX_RED 12
#define MAX_GREEN 13
#define MAX_BLUE 14

#define GAME_SET_ELEMENTS 3
#define GAME_SETS 10

#define CUBE_BLUE "blue"
#define CUBE_RED "red"
#define CUBE_GREEN "green"

typedef struct {
    int blue_cubes;
    int green_cubes;
    int red_cubes;
} set_t;

typedef struct {
    int id;
    set_t sets[GAME_SETS];
} game_t;

set_t init_set(void) {
    set_t result;
    result.green_cubes = 0;
    result.red_cubes = 0;
    result.blue_cubes = 0;
    return result;
}

// TODO: i don't know how to init 'set' here
game_t init_game(void) {
    game_t game;
    game.id = 0;
    return game;
}

// Example of a line:
// "Game 1: 1 blue, 1 green; 3 red; 15 blue, 3 green"
game_t parse_game(char *line) {
    game_t game = init_game();

    char *game_parts[2];
    split_to_array(line, ":", game_parts);

    char *game_id_part[2]; // should be 'Game' '1'; so only two elements
    split_to_array(game_parts[0], " ", game_id_part);

    char *game_id = game_id_part[1];
    game.id = atoi(game_id); // TODO: how does atoi work if the ptr value is not a nr?

    char *game_sets[GAME_SETS];
    int game_sets_len = split_to_array(game_parts[1], ";", game_sets);

    for (int i = 0; i < game_sets_len; ++i) {
        set_t set = init_set();

        char *set_array[GAME_SET_ELEMENTS];
        int set_array_len = split_to_array(game_sets[i], ",", set_array);

        for (int j = 0; j < set_array_len; ++j) {
            char *set_elements[GAME_SET_ELEMENTS];
            split_to_array(set_array[j], " ", set_elements);

            if (strcmp(set_elements[1], CUBE_BLUE) == 0) {
                set.blue_cubes = atoi(set_elements[0]);
            } else if (strcmp(set_elements[1], CUBE_RED) == 0) {
                set.red_cubes = atoi(set_elements[0]);
            } else if (strcmp(set_elements[1], CUBE_GREEN) == 0) {
                set.green_cubes = atoi(set_elements[0]);
            } else {
                continue;
            }
        }

        game.sets[i] = set;
    }

    return game;
}

// NOTE: part two logic here
int part_two(char *line) {
    game_t game = parse_game(line);

    for (int i = 0; i < 10; ++i) {
        set_t set = game.sets[i];

        if (set.blue_cubes > MAX_BLUE
            || set.red_cubes > MAX_RED
            || set.green_cubes > MAX_GREEN) {
            continue;
        }
    }

    return 0;
}

int solve_day_two() {
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
