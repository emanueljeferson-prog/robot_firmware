#ifndef GPIO_INTERFACE_H
#define GPIO_INTERFACE_H

#include <stdint.h>

void init(const uint8_t pin);
void setOutput(const uint8_t pin);
void setInput(const uint8_t pin); 
void setInterruptRequestWithCallback(const uint8_t pin, void* callback); 

#endif // GPIO_INTERFACE_H