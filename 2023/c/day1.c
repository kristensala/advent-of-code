#include <ctype.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STR_LEN 200
#define BUFSIZE 32*1024

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

int main(void) {
    char input[BUFSIZE];
    FILE *fp = fopen("./inputs/day1-1.prod", "r");
    size_t inputLen = fread(input, 1, BUFSIZE, fp);
    fclose(fp);
    
    partOne(input, inputLen);

    return 0;
}


