#include <cstdint>
#include <cstdlib>
#include <solid_log.h>
#include <kernel.h>

namespace {
namespace green_led {

constexpr std::uintptr_t GPIO_BASE = 0xFE200000UL;
constexpr std::size_t GPIO_NUM = 42;

void init()
{
    auto reg = reinterpret_cast<volatile std::uint32_t *>(GPIO_BASE + ((GPIO_NUM / 10) << 2)); // GPFSEL4
    int mode = 1; // output
    *reg = (*reg & ~(7 << ((GPIO_NUM % 10) * 3))) | (mode << ((GPIO_NUM % 10) * 3));
}

void update(bool new_state)
{
    auto reg = reinterpret_cast<volatile std::uint32_t *>(GPIO_BASE + ((GPIO_NUM / 32) << 2)
        + (new_state ? 0x1c /* GPSET1 */ : 0x28 /* GPCLR1 */));
    *reg |= 1 << (GPIO_NUM % 32);
}

} // namespace green_led
} // namespace

extern "C" void slo_main()
{
    SOLID_LOG_printf("Starting LED blinker\n");

    // Configure the LED port
    green_led::init();

    while (true) {
        // Turn on the LED
        green_led::update(true);
        dly_tsk(200'000);

        // Turn off the LED
        green_led::update(false);
        dly_tsk(200'000);
    }
}
