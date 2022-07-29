#include <cstdint>
#include <cstdlib>
#include <solid_log.h>
#include <kernel.h>

namespace {

constexpr std::uintptr_t GPIO_BASE = 0xFE200000UL;
constexpr std::size_t GPIO_NUM = 42;

void green_led_prepare()
{
    auto reg = reinterpret_cast<volatile std::uint32_t *>(GPIO_BASE + ((GPIO_NUM / 10) << 2)); // GPFSEL4
    int mode = 1; // output
    *reg = (*reg & ~(7 << ((GPIO_NUM % 10) * 3))) | (mode << ((GPIO_NUM % 10) * 3));
}

void green_led_light(bool new_state)
{
    auto reg = reinterpret_cast<volatile std::uint32_t *>(GPIO_BASE + ((GPIO_NUM / 32) << 2)
        + (new_state ? 0x1c /* GPSET1 */ : 0x28 /* GPCLR1 */));
    *reg |= 1 << (GPIO_NUM % 32);
}

} // namespace

extern "C" void slo_main()
{
    SOLID_LOG_printf("Starting LED blinker\n");
    green_led_prepare();
    while (true) {
        green_led_light(false);
        dly_tsk(200'000);
        green_led_light(true);
        dly_tsk(200'000);
    }
}
