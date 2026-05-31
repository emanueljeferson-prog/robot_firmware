#include "gpio.h"
#include <hardware/gpio.h>
#include <hardware/irq.h>

void init(const uint8_t pin)
{
    gpio_init(pin);
}

void setOutput(const uint8_t pin) 
{
    gpio_set_dir(pin, GPIO_OUT);
}

void setInput(const uint8_t pin)
{
    gpio_set_dir(pin, GPIO_IN);
}

void setInterruptRequestWithCallback(const uint8_t pin, void* callback)
{
    gpio_set_irq_enabled_with_callback(pin, GPIO_IRQ_EDGE_RISE, true, callback)
}
