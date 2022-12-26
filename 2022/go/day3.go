package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

func main() {
    items := "abcdefghijklmnopqrstuvwxyz"
    
    fileContent, _ := os.ReadFile("../files/day3.txt")
    
    var result []rune
    if fileContent != nil {
        lines := strings.Split(string(fileContent), "\n")
        
        for _, line := range lines {
            firstCompartment := line[:len(line) / 2]
            secondCompartment := line[len(line) / 2:]

            var lineOcurrence []rune
            for _, ch := range firstCompartment {
                if strings.Contains(secondCompartment, string(ch)) {
                    if !strings.Contains(string(lineOcurrence), string(ch)) {
                        lineOcurrence = append(lineOcurrence, ch)
                    }
                }
            }

            result = append(result, lineOcurrence...)
        }

        getPoints(string(result), items)

        part2(lines, items)
    }

}

func getPoints(result string, pointsSystem string) {
    points := 0
    for _, ch := range result {
        if unicode.IsUpper(ch) {
            lowerChar := unicode.ToLower(ch)
            fmt.Println(strings.Index(pointsSystem, string(lowerChar)) + 27)
            points += strings.Index(pointsSystem, string(lowerChar)) + 27
        } else {
            fmt.Println(strings.Index(pointsSystem, string(ch)) + 1)
            points += strings.Index(pointsSystem, string(ch)) + 1
        }
    }

    fmt.Println(points)
}

func part2(ruckSacks []string, items string) {
    var groups [][]string

    chunkSize := 3
    for i := 0; i < len(ruckSacks); i += chunkSize {
        end := i + chunkSize

        if end > len(ruckSacks) {
            end = len(ruckSacks)
        }            

        groups = append(groups, ruckSacks[i:end])
    }

    fmt.Println("%#v\n", groups)

    var res string
    for _, group := range groups {
        res += string(getBadge(group))
    }

    getPoints(res, items)
}

func getBadge(group []string) rune {
    if len(group) == 3 {
        ruckSack_1 := group[0]
        ruckSack_2 := group[1]
        ruckSack_3 := group[2]

        for _, ch := range ruckSack_1 {
                if strings.Contains(string(ruckSack_2), string(ch)) && strings.Contains(string(ruckSack_3), string(ch)) {
                    return ch
                }

        }
        return ' '
    }
    return ' '
}
