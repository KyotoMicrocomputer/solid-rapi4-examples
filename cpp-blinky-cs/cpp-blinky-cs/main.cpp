﻿#include <cstdint>
#include <cstdlib>
#include <solid_cs_assert.h>
#include <solid_log.h>
#include <solid_timer.h>

namespace {
namespace green_led {

constexpr std::uintptr_t GPIO_BASE = 0xFE200000UL;
constexpr std::size_t GPIO_NUM = 42;

void init()
{
    auto reg = reinterpret_cast<volatile std::uint32_t *>(
        GPIO_BASE + 0x00 /* GPFSEL0 */
        + ((GPIO_NUM / 10) * 4));
    int mode = 1; // output
    *reg = (*reg & ~(7 << ((GPIO_NUM % 10) * 3))) | (mode << ((GPIO_NUM % 10) * 3));
}

void update(bool new_state)
{
    auto reg = reinterpret_cast<volatile std::uint32_t *>(
        GPIO_BASE + (new_state ? 0x1c /* GPSET0 */ : 0x28 /* GPCLR0 */)
        + ((GPIO_NUM / 32) * 4));
    *reg = 1 << (GPIO_NUM % 32);
}

} // green_led

// The timer state (owned by the system after registration)
SOLID_TIMER_HANDLER g_timer;

// Tracks the latest LED state
bool g_led_state = false;

} // namespace

extern "C" void slo_main()
{
    SOLID_LOG_printf("Starting LED blinker\n");

    // Configure the LED port
    green_led::init();

    // Initialize the timer object
    g_timer.type = SOLID_TIMER_TYPE_INTERVAL;
    g_timer.time = 200'000;
    g_timer.func = [] (void *, SOLID_CPU_CONTEXT *) {
        // Determine the next LED state
        g_led_state = !g_led_state;

        // Toggle the LED
        green_led::update(g_led_state);
    };
    g_timer.param = NULL;

    // Start the timer
    int ret = SOLID_TIMER_RegisterTimer(&g_timer);
    solid_cs_assert(ret == SOLID_ERR_OK);
}
