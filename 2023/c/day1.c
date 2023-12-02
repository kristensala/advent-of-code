#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STR_LEN 80

typedef struct NumberType {
    char name[10];
    int value;
} numberType;

int parseSpelledNumbers(char *line) {
    numberType nOne, nTwo, nThree, nFour, nFive, nSix, nSeven, nEight, nNine;
    strcpy(nOne.name, "one");
    nOne.value = 1;

    strcpy(nTwo.name, "two");
    nOne.value = 2;

    strcpy(nThree.name, "three");
    nOne.value = 3;

    strcpy(nFour.name, "four");
    nOne.value = 4;

    strcpy(nFive.name, "five");
    nOne.value = 5;

    strcpy(nSix.name, "six");
    nOne.value = 6;

    strcpy(nSeven.name, "seven");
    nOne.value = 7;

    strcpy(nEight.name, "eight");
    nOne.value = 8;

    strcpy(nNine.name, "nine");
    nOne.value = 9;

    numberType numberList[9] = {nOne, nTwo, nThree, nFour, nFive, nSix, nSeven, nEight, nNine};


    //TODO:
    for (int i = 3; i < strlen(line); i++) {
        for (int j = 0; j < 9; j++) {
        }
    }

    return 0;
}

int main(void)
{

    FILE *fp = fopen("../c/inputs/day1-2.test", "r");
    if (fp == NULL) {
        exit(EXIT_FAILURE);
    }

    // +1 is a commont practice among C devs
    // Becuse every string is terminated by a null character,
    // meaning that if there is no room for the null char
    // then it may cause unpredictable results
    //
    // Strings are null terminated??? (don't know what that means)
    char line[STR_LEN + 1];
    char nrString[200];

    int resultPart1;
    int resultPart2;

    while (fgets(line, sizeof(line), fp)) {
        parseSpelledNumbers(line);

        int pos = 0;
        for(int i = 0; i < strlen(line); i++) {
            char t = line[i];

            if (isdigit(t) && !isspace(t)) {
                pos += sprintf(&nrString[pos], "%c", t);
            }

            if (i == strlen(line) - 1 && strlen(nrString) > 0) {
                char buffer[30];
                sprintf(buffer, "%c%c", nrString[0], nrString[strlen(nrString) - 1]);
                resultPart1 += atoi(buffer);
                //printf("number to calc with %s\n", buffer);
            }
        }

//        printf("result: %s", nrString);

    }

    printf("Result part 1: %d", resultPart1);

    fclose(fp);

    return 0;
}
