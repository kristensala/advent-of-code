package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
    bytes, error := os.ReadFile("../files/day1.txt")

    if error != nil {
        fmt.Println(error)
        return
    }

    elvesArray := strings.Split(string(bytes), "\n\n")

    for _, elf := range elvesArray {
        basket := strings.Split(elf, ",")
        fmt.Println(elf)

        for _, item := range basket {
            fmt.Println("item", item)
        }
    }
}
