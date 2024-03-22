cmake_minimum_required(VERSION 3.22)
project(prejil C)

set(CMAKE_C_STANDARD 17)

add_executable(prejil src/main.c
        src/opt/help.c
        src/opt/subopt.h
)
