cmake_minimum_required(VERSION 3.16)

project(gpio_interface C CXX ASM)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

add_library(gpio_interface STATIC
    gpio.c
)

target_include_directories(gpio_interface PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}
)

target_link_libraries(gpio_interface
    pico_stdlib
    hardware_gpio
)