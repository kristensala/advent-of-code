package main

import "core:fmt"
import "core:strings"
import "core:strconv"
import "core:os"
import "core:text/regex"

main :: proc() {
    byte_data, ok := os.read_entire_file("./input/day3.prod", context.allocator)

    if !ok {
        fmt.print("Could not read the file!")
        return;
    }

    defer delete(byte_data, context.allocator)
    data := string(byte_data)

    array: [dynamic]int
    defer delete(array)

    for line in strings.split_lines_iterator(&data) {
        //filter_multiplications(line, &array)
        part_two(line, &array)
    }

    part_one_result: int
    for num in array {
        part_one_result += num
    }

    fmt.println(part_one_result)
}

@(private="file")
part_two :: proc(corrupt_line: string, array: ^[dynamic]int) {
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
    dont_regex_result, dont_ok := regex.match_and_allocate_capture(dont_regex, corrupt_line) 

    if !dont_ok || (!dont_ok && !do_ok) {
        filter_multiplications(corrupt_line, array)
        return
    }

    if dont_regex_result.pos[0][0] < do_regex_result.pos[0][0] {
        new_line, ok := strings.replace(
            corrupt_line,
            corrupt_line[dont_regex_result.pos[0][0]:do_regex_result.pos[0][1]],
            "", 
            1
        )

        if !ok {
            filter_multiplications(new_line, array)
            return
        }

        part_two(new_line, array)

    } else {
        replaced_do, is_ok := strings.replace(
            corrupt_line,
            corrupt_line[do_regex_result.pos[0][0]:do_regex_result.pos[0][1]],
            "",
            1
        )

        if !is_ok {
            filter_multiplications(replaced_do, array)
            return;
        }

        part_two(replaced_do, array)
    }
}

@(private="file")
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
