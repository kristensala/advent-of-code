package main

import (
	"fmt"
	"os"
	"strings"
)

// First column - enemy
//A - Rock
//B - Paper
//C - Scissors

// Second column - my choise
// X - Rock
// Y - Paper
// Z - Scissors

// Rock 1; Paper 2; Scissors 3
// Lost 0; Draw 3; Win 6
func main() {
    fileContent, _ := os.ReadFile("../files/day2.txt")
    lines := strings.Split(string(fileContent), "\n")

    handPoints := map[string]int {
        "X": 1,
        "Y": 2,
        "Z": 3,
    }


    // Part 1 points
    myGamePoints := map[string]int {
        "A X": 3,
        "A Y": 6,
        "A Z": 0,
        "B X": 0,
        "B Y": 3,
        "B Z": 6,
        "C X": 6,
        "C Y": 0,
        "C Z": 3,
    }

    selectResult := map[string]map[int]string {
        "A": {
            3: "A X",
            6: "A Y",
            0: "A Z",
        },
        "B": {
            3: "B Y",
            6: "B Z",
            0: "B X",
        },
        "C": {
            3: "C Z",
            6: "C X",
            0: "C Y",
        },
    }

    decision := map[string]int {
        "X": 0,
        "Y": 3,
        "Z": 6,
    }


    totalPoints := 0
    totalPointsPart2 := 0

    for _, game := range lines {
        if len(game) > 0 {
            totalPoints += myGamePoints[game]
            totalPoints += handPoints[strings.Split(string(game), " ")[1]]


            roundPoints := decision[strings.Split(string(game), " ")[1]]
            selection := selectResult[strings.Split(string(game), " ")[0]][roundPoints] 
            myNewHand := handPoints[strings.Split(string(selection), " ")[1]];

            totalPointsPart2 += roundPoints
            totalPointsPart2 += myNewHand
        }
    }

    fmt.Println(totalPoints)
    fmt.Println(totalPointsPart2)

    // PArt2 
    // X - i need to lose
    // Y - draw
    // Z - win
}

