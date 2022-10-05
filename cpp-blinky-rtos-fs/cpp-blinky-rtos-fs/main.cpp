#include <algorithm>
#include <atomic>
#include <array>
#include <cstdint>
#include <cstdlib>
#include <solid_cs_assert.h>
#include <solid_log.h>
#include <solid_fs.h>
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
    *reg = 1 << (GPIO_NUM % 32);
}

} // namespace green_led

std::int32_t read_requested_power()
{
    int fd;
    int ret = SOLID_FS_Open(&fd, R"(\OSCOM_FS\etc\led-power.txt)", O_RDONLY);
    if (ret != SOLID_ERR_OK) {
        goto io_error;
    }

    std::array<char, 64> buf;
    size_t num_bytes_read;
    ret = SOLID_FS_Read(fd, buf.data(), buf.size() - 1, &num_bytes_read);
    SOLID_FS_Close(fd);
    if (ret != SOLID_ERR_OK) {
        goto io_error;
    }

    buf[num_bytes_read] = 0;

    // Parse the value
    long value;
    value = strtol(buf.data(), nullptr, 10);

    // Clamp it to [0, 100]
    return static_cast<std::int32_t>(std::max<long>(std::min<long>(value, 100), 0));
   
io_error:
    if (ret == SOLID_ERR_NOTFOUND) {
        SOLID_LOG_printf("failed to read from /etc/led-power.txt: not found\n");
    } else {
        SOLID_LOG_printf("failed to read from /etc/led-power.txt: %d\n", ret);
    }
    return -1;
}

std::atomic<std::uint32_t> g_led_power;

void led_power_update_task_entry(intptr_t)
{
    std::int32_t smoothed_power = 0;
    std::int32_t default_power = 0;

    for (;; default_power ^= 100) {
        // Read the requested LED power every second
        std::int32_t power = read_requested_power();

        if (power < 0) {
            // Blink the LED by default
            power = default_power;
        }

        for (int frame = 0; frame < 100; ++frame) {
            if (power > smoothed_power) {
                smoothed_power = std::min(smoothed_power + 1, power);
            } else {
                smoothed_power = std::max(smoothed_power - 1, power);
            }

            // Apply gamma conversion and map to the range [0, 0xffff'ffff] (approx.)
            const std::int32_t corrected_power = smoothed_power * smoothed_power * 429'496;

            g_led_power.store(static_cast<std::uint32_t>(corrected_power), std::memory_order_relaxed);

            dly_tsk(10'000); // 10ms
        }
    }
}

} // namespace

extern "C" void slo_main()
{
    SOLID_LOG_printf("Starting LED blinker\n");

    // Configure the LED port
    green_led::init();

    // Start a task
    const T_CTSK led_power_update_task = {
        .tskatr = TA_ACT,
        .exinf = 0,
        .task = led_power_update_task_entry,
        .itskpri = 10,
        .stksz = 4096,
        .stk = NULL,
        .iprcid = 1,
        .affinity = uint_t(1) << 0,
    };
    int ret = acre_tsk(&led_power_update_task);
    solid_cs_assert(ret > 0);

    // Migrate to the second core, where there are fewer activities
    ret = mig_tsk(TSK_SELF, 2);
    solid_cs_assert(ret == E_OK);

    // Software delta-sigma DAC
    std::uint32_t integrator = 0;

    while (true) {
        const std::uint32_t new_value = integrator + g_led_power.load(std::memory_order_relaxed);
        const bool output = new_value < integrator;
        integrator = new_value;

        green_led::update(output);

        dly_tsk(100); // 100μs
    }
}
