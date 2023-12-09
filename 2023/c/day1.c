#include <ctype.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STR_LEN 200
#define BUFSIZE 32*1024

typedef struct {
    char value;
    int pos;
} indexer;

void partOne(char *input, size_t inputLength) {
    int result = 0;

    char firstDigit = '\0';
    char lastDigit = '\0';

    for (int i = 0; i < inputLength; i++) {
        char character = input[i];

        if (character == '\n') {
            char *number;
            number = malloc(2 + 1);
            strcpy(number, &firstDigit);
            strcat(number, &lastDigit);

            result += atoi(number);
            free(number);

            firstDigit = '\0';
            lastDigit = '\0';
            continue;
        }

        if (isdigit(character)) {
            if (firstDigit == '\0') {
                firstDigit = character;
            }

            lastDigit = character;
        }
    }

    printf("Part One: %i", result);
}

char *substring(char *sub, char *input, int start, int end) {
    int c = 0;
    while (c < end) {
        sub[c] = input[start + c];
        c++;
    }

    sub[c] = '\0';
    return sub;
}

void getNumber(indexer *indexer, int pos, char *string) {
    const char mapping[9][3][10] = {
        { "one", "1"},
        { "two", "2"},
        { "three", "3"},
        { "four", "4"},
        { "five", "5"},
        { "six", "6"},
        { "seven", "7"},
        { "eight", '8'},
        { "nine", "9"}
    };

    for (int i = 0; i < 9; i++) {
        char name[10];
        strcpy(name, mapping[i][0]);

        char value[10];
        strcpy(value, mapping[i][1]);

        char *ptr = strstr(string, name);
        if (ptr != NULL) {
            pos = ptr - string;

            indexer->value = *value;
            indexer->pos = pos;
        }
    }
}

void partTwo(char *input, size_t inputLength) {
    int result = 0;
    char *numbers = malloc(32 * 1024 + 1);

    if (numbers == NULL) {
        printf("failed to allocate memory for numbers");
        exit(-1);
    }

    for (int i = 0; i < inputLength; i++) {
        char character = input[i];

        if (character == '\n') {
            // TODO: I do not know what is happening here
            // Getting characters mixed with numbers, but should not
            printf("NUMBERS: %s \n", numbers);
            strcpy(numbers, "\0");

            continue;
        }

        if (isdigit(character)) {
            strcat(numbers, &character); // this is no bueno getting values like 3L, 2K etc etc
        } else {
            indexer indexer;
            indexer.value = '0';
            indexer.pos = 0;

            char *sub = malloc(6 * sizeof(char));
            if (sub == NULL) {
                printf("Failed to allocate memory for substring");
                exit(-1);
            }

            substring(sub, input, i, 5);

            int pos = 0;
            getNumber(&indexer, pos, sub);
            free(sub);

            if (indexer.value != '0') {
                printf("found: %c on pos %d; new idx: %d\n", indexer.value, indexer.pos, indexer.pos + i);
                strcat(numbers, &indexer.value);
                i = i + indexer.pos;
            }
        }
    }

    free(numbers);

    printf("REsult: %d", result);
}

void translateTest(char *input, size_t inputLength) {
    int wordLength = 5;
    for (int i = 0; i < inputLength; i++) {
        indexer indexer;
        indexer.value = '0';
        indexer.pos = 0;

        char *sub = malloc(6 * sizeof(char));
        substring(sub, input, i, wordLength);

        int pos = 0;
        getNumber(&indexer, pos, sub);
        free(sub);

        if (indexer.value != '0') {
            printf("found: %c on pos %d; new idx: %d\n", indexer.value, indexer.pos, indexer.pos + i);
            i = i + indexer.pos;
        }
    }
}

int main(void) {
    char input[BUFSIZE];
    FILE *fp = fopen("./inputs/day1-2.test", "r");
    size_t inputLen = fread(input, 1, BUFSIZE, fp);
    fclose(fp);
    
    //partOne(input, inputLen);
    //translateTest(input, inputLen);
    //
    partTwo(input, inputLen);

    return 0;
}


