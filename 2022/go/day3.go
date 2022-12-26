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

            fmt.Println(firstCompartment)
            fmt.Println(secondCompartment)
            
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

        fmt.Println(string(result)) 

        getPoints(string(result), items)
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
