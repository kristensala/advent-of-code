package main

import "core:fmt"
import "core:strings"
import "core:os"
import "core:strconv"
import "core:math"

main :: proc() {
    byte_data, ok := os.read_entire_file("./input/day2", context.allocator)

    if !ok {
        fmt.print("Could not read the file!")
        return;
    }

    defer delete(byte_data, context.allocator)
    data := string(byte_data)


    safe_count := 0;

    report: [dynamic]int
    for line in strings.split_lines_iterator(&data) {
        report_str := strings.split(line, " ")

        for level in report_str {
            level_nr := strconv.atoi(level)
            append(&report, level_nr)
        }
        if is_safe(report) {
            safe_count += 1
        } else {
            if try_dampen(report) {
                safe_count += 1
            }
        }

        clear(&report)
    }

    fmt.println(safe_count)
}

try_dampen :: proc(unsafe_report: [dynamic]int) -> bool {
    for i := 0; i <= len(unsafe_report) - 1; i += 1 {
        array_copy := make([dynamic]int, len(unsafe_report), cap(unsafe_report))
        defer delete(array_copy)
        copy(array_copy[:], unsafe_report[:])

        ordered_remove(&array_copy, i)
        if is_safe(array_copy) {
            return true;
        }
    }

    return false
}

is_safe :: proc(report: [dynamic]int) -> bool {
    is_increase := false;

    for i := 0; i < len(report); i += 1 {
        if i == len(report) - 1 {
            break
        }

        current_level := report[i]
        next_level := report[i + 1]

        if i == 0 {
            if next_level > current_level {
                is_increase = true
            }
        }

        if is_increase && next_level < current_level {
            return false;
        }

        if !is_increase && next_level > current_level {
            return false
        }

        if current_level == next_level {
            return false
        }

        diff := math.abs(current_level - next_level)
        if diff > 3 {
            return false;
        }
    }
    return true;
}
