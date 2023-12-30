cmake_minimum_required(VERSION 3.27)
project(aoc C)

set(CMAKE_C_STANDARD 99)

configure_file(${CMAKE_CURRENT_SOURCE_DIR}/inputs/day2-1.test
    ${CMAKE_CURRENT_BINARY_DIR} COPYONLY)

add_executable(aoc main.c
        aoc.h
        day2.c
)