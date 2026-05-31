cmake_minimum_required(VERSION 3.16)

include(pico_sdk_import.cmake)

project(robot_firmware C CXX ASM)

pico_sdk_init()

add_subdirectory(src/hal/gpio)