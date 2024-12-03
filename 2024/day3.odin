package main

import "core:fmt"
import "core:strings"
import "core:strconv"
import "core:os"
import "core:text/regex"

main :: proc() {
    byte_data, ok := os.read_entire_file("./input/day3.test", context.allocator)

    if !ok {
        fmt.print("Could not read the file!")
        return;
    }

    defer delete(byte_data, context.allocator)
    data := string(byte_data)

    safe_count := 0;
    array: [dynamic]int
    defer delete(array)

    for line in strings.split_lines_iterator(&data) {
        //filter_multiplications(line, &array)
        part_two(line)
    }

    part_one_result: int
    for num in array {
        part_one_result += num
    }

    fmt.println(part_one_result)
}

part_two :: proc(corrupt_line: string) {
    do_regex, do_err := regex.create_by_user(`/do\(\)/g`)
    if do_err != nil {
        fmt.println(do_err)
        return 
    } 

    dont_regex, dont_err := regex.create_by_user(`/don't\(\)/g`)
    if dont_err != nil {
        fmt.println(dont_err)
        return 
    }

    do_regex_result, do_ok := regex.match_and_allocate_capture(do_regex, corrupt_line) 
    if !do_ok { 
        return 
    }

    dont_regex_result, dont_ok := regex.match_and_allocate_capture(dont_regex, corrupt_line) 
    if !dont_ok { 
        return 
    }

    fmt.println(do_regex_result)
    fmt.println(dont_regex_result)
}

filter_multiplications :: proc(corrupt_line: string, array: ^[dynamic]int) {
    reg, err := regex.create_by_user(`/mul\(\d+,\d+\)/g`)
    if err != nil {
        fmt.println(err)
        return 
    } 

    res, ok := regex.match_and_allocate_capture(reg, corrupt_line) 
    if !ok { 
        return 
    }

    if len(corrupt_line) < res.pos[0][1] {
        return 
    }

    result := corrupt_line[res.pos[0][0]: res.pos[0][1]]
    values := strings.split(result, ",")

    first_number := strconv.atoi(values[0][4:])
    second_number := strconv.atoi(values[1][:len(values[1]) - 1])

    mul := first_number * second_number
    append(array, mul)

    filter_multiplications(corrupt_line[res.pos[0][1]:], array) 
}
