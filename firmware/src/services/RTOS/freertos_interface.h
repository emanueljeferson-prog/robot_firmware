#ifndef FREE_RTOS_INTERFACE_H
#define FREE_RTOS_INTERFACE_H

#include "FreeRTOS.h"
#include "task.h"
#include "semphr.h"

void createTask(void* task_ptr, const char* const task_name, const uint16_t stack_size, uint8_t priority, void* parameters);
void schedulerStart();
void delayTask(const uint32_t delay);

#endif // FREE_RTOS_INTERFACE_H 