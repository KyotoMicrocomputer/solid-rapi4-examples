#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <solid_log.h>
#include <kernel.h>

#define GPIO_BASE       0xFE200000UL
#define GPIO_NUM        42

void green_led_init()
{
    volatile uint32_t *reg = (volatile uint32_t *)(
        GPIO_BASE + 0x00 /* GPFSEL0 */
        + ((GPIO_NUM / 10) * 4));
    int mode = 1; // output
    *reg = (*reg & ~(7 << ((GPIO_NUM % 10) * 3))) | (mode << ((GPIO_NUM % 10) * 3));
}

void green_led_update(bool new_state)
{
    volatile uint32_t *reg = (volatile uint32_t *)(
        GPIO_BASE + (new_state ? 0x1c /* GPSET0 */ : 0x28 /* GPCLR0 */)
        + ((GPIO_NUM / 32) * 4));
    *reg = 1 << (GPIO_NUM % 32);
}

void slo_main()
{
    SOLID_LOG_printf("Starting LED blinker\n");

    // Configure the LED port
    green_led_init();

    while (true) {
        // Turn on the LED
        green_led_update(true);
        dly_tsk(200000);

        // Turn off the LED
        green_led_update(false);
        dly_tsk(200000);
    }
}
