#include <cstdint>
#include <cstdlib>
#include <solid_cs_assert.h>
#include <solid_log.h>
#include <solid_intc.h>

namespace {

inline volatile uint32_t& reg32(std::uintptr_t va) {
    return *reinterpret_cast<volatile std::uint32_t *>(va);
}

namespace green_led {

constexpr std::uintptr_t GPIO_BASE = 0xFE200000UL;
constexpr std::size_t GPIO_NUM = 42;

void init()
{
    auto& reg = reg32(GPIO_BASE + ((GPIO_NUM / 10) << 2)); // GPFSEL4
    int mode = 1; // output
    reg = (reg & ~(7 << ((GPIO_NUM % 10) * 3))) | (mode << ((GPIO_NUM % 10) * 3));
}

void update(bool new_state)
{
    auto& reg = reg32(GPIO_BASE + ((GPIO_NUM / 32) << 2)
        + (new_state ? 0x1c /* GPSET1 */ : 0x28 /* GPCLR1 */));
    reg = 1 << (GPIO_NUM % 32);
}

} // green_led

namespace ap804 {
    
constexpr std::uintptr_t ARM_TIMER_BASE = 0xFE00B000UL;
constexpr std::uintptr_t ARM_TIMER_LOAD = ARM_TIMER_BASE + 0x400;
constexpr std::uintptr_t ARM_TIMER_CONTROL = ARM_TIMER_BASE + 0x408;
constexpr std::uintptr_t ARM_TIMER_IRQCNTL = ARM_TIMER_BASE + 0x40C;
constexpr std::uintptr_t ARM_TIMER_RELOAD = ARM_TIMER_BASE + 0x418;
constexpr std::uintptr_t ARM_TIMER_PREDIV = ARM_TIMER_BASE + 0x41C;

constexpr int INTNO = 64;

void init(uint32_t load)
{
    reg32(ARM_TIMER_CONTROL) = ((0x3E & 0xFF) << 16) // [23:16] FREEDIV
                             | ((0x00 & 0x01) << 9)    // [9] ENAFREE
                             | ((0x00 & 0x01) << 8)    // [8] DBGHALT
                             | ((0x00 & 0x01) << 7)    // [7] ENABLE
                             | ((0x00 & 0x01) << 5)    // [5] IE
                             | ((0x00 & 0x03) << 2)    // [3:2] DIV
                             | ((0x01 & 0x01) << 1);   // [1] 32BIT
    reg32(ARM_TIMER_LOAD) = load;
    reg32(ARM_TIMER_RELOAD) = load;
    reg32(ARM_TIMER_PREDIV) = 0x7d;
    reg32(ARM_TIMER_IRQCNTL) = 0;
}

void start()
{
    reg32(ARM_TIMER_CONTROL) = ((0x3E & 0xFF) << 16) // [23:16] FREEDIV
                             | ((0x00 & 0x01) << 9)    // [9] ENAFREE
                             | ((0x00 & 0x01) << 8)    // [8] DBGHALT
                             | ((0x01 & 0x01) << 7)    // [7] ENABLE
                             | ((0x01 & 0x01) << 5)    // [5] IE
                             | ((0x00 & 0x03) << 2)    // [3:2] DIV
                             | ((0x01 & 0x01) << 1);   // [1] 32BIT
}

void clear_int()
{
    reg32(ARM_TIMER_IRQCNTL) = 1;
}

} // ap804

// The interrupt handler (read by the system after registration)
SOLID_INTC_HANDLER g_handler;

// Tracks the latest LED state
bool g_led_state = false;

} // namespace

extern "C" void slo_main()
{
    SOLID_LOG_printf("Starting LED blinker\n");

    // Configure the LED port
    green_led::init();

    // Configure the AP804 instance
    ap804::init(1'000'000);

    // Initialize the interrupt handler object for the AP804 interrupt line
    g_handler.intno = ap804::INTNO;
    g_handler.priority = 10;
    g_handler.config = -1;
    g_handler.func = [](void *, SOLID_CPU_CONTEXT *) {
        // Clear the AP804 instance's interrupt flag
        ap804::clear_int();

        // Determine the next LED state
        g_led_state = !g_led_state;

        // Toggle the LED
        green_led::update(g_led_state);

        return 0;
    };
    g_handler.param = NULL;

    // Register the interrupt handler object for the AP804 interrupt line
    int ret = SOLID_INTC_Register(&g_handler);
    solid_cs_assert(ret == SOLID_ERR_OK);

    // Enable the AP804 interrupt line
    ret = SOLID_INTC_Enable(g_handler.intno);
    solid_cs_assert(ret == SOLID_ERR_OK);

    // Start the AP804 timer
    ap804::start();
}
