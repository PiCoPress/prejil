cmake_minimum_required(VERSION 3.22)
project(prejil C)

set(CMAKE_C_STANDARD 17)

include_directories(comline/opt)

add_executable(prejil
        comline/opt/help.c
        comline/opt/subopt.h
        comline/main.c)