package main

import "core:fmt"
import "core:os"
import "core:strings"
import "core:strconv"
import "core:slice"
import "core:math"

main :: proc() {
    byte_data, ok := os.read_entire_file("./input/day1-1", context.allocator)

    if !ok {
        fmt.print("Could not read the file!")
        return;
    }

    defer delete(byte_data, context.allocator)

    data := string(byte_data)

    list_one: [dynamic]int
    defer delete(list_one)

    list_two: [dynamic]int
    defer delete(list_two)

    for line in strings.split_lines_iterator(&data) {
        location_ids := strings.split(line, "   ") // why does \t not work??

        append(&list_one, strconv.atoi(location_ids[0]))
        append(&list_two, strconv.atoi(location_ids[1]))
    }

    slice.sort(list_one[:])
    slice.sort(list_two[:])

    result: int
    for i := 0; i < len(list_one); i += 1 {
        result += math.abs(list_one[i] - list_two[i])
    }

    fmt.println(result)

    result_two := part_two(list_one, list_two)
    fmt.println(result_two)
}

part_two :: proc(left_list: [dynamic]int, right_list: [dynamic]int) -> int {
    occurence: int
    part_two_result: int

    for left_id in left_list {
        occurence = 0

        for right_id in right_list {
            if left_id == right_id {
                occurence += 1
            }
        }

        part_two_result += left_id * occurence
    }
    return part_two_result
}

