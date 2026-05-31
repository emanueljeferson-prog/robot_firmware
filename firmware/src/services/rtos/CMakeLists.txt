cmake_minimum_required(VERSION 3.16)
project(freertos_interface C)

set(CMAKE_C_STANDARD 11)

set(FREERTOS_KERNEL_PATH ${CMAKE_CURRENT_SOURCE_DIR}/FreeRTOS-Kernel)

file(GLOB FREERTOS_POSIX_UTILS
    ${FREERTOS_KERNEL_PATH}/portable/ThirdParty/GCC/Posix/utils/*.c
)

add_library(freertos_interface STATIC
    ${CMAKE_CURRENT_SOURCE_DIR}/freertos_interface.c
    ${FREERTOS_KERNEL_PATH}/tasks.c
    ${FREERTOS_KERNEL_PATH}/queue.c
    ${FREERTOS_KERNEL_PATH}/list.c
    ${FREERTOS_KERNEL_PATH}/timers.c
    ${FREERTOS_KERNEL_PATH}/portable/MemMang/heap_4.c
    ${FREERTOS_KERNEL_PATH}/portable/ThirdParty/GCC/Posix/port.c
    ${FREERTOS_POSIX_UTILS}
)

target_include_directories(freertos_interface PUBLIC
    ${FREERTOS_KERNEL_PATH}/include
    ${FREERTOS_KERNEL_PATH}/portable/ThirdParty/GCC/Posix
    ${CMAKE_CURRENT_SOURCE_DIR}
)

find_package(Threads REQUIRED)
target_link_libraries(freertos_interface PUBLIC Threads::Threads)