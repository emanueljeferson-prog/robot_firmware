#include "freertos_interface.h"
#include "FreeRTOS.h"
#include "task.h"
#include <stdio.h>

void createTask(void* task_ptr, const char* const task_name, const uint16_t stack_size, uint8_t priority, void* parameters)
{
    printf("Task name: %s\n", task_name);
    xTaskCreate(
        task_ptr,
        task_name,
        stack_size,
        parameters,
        priority,
        NULL
    );
}

void schedulerStart() {
    vTaskStartScheduler();
}