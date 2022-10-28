//! CPU usage monitor
use solid::{singleton::pin_singleton, thread::CpuCx, timer};
use std::sync::atomic;

pub fn init() {
    // Construct a timer object in a global variable.
    let Ok(handler) = pin_singleton!(: Timer<_> = timer::Timer::new(
        timer::Schedule::Interval(timer::Usecs32(1_000)),
        timer_handler,
    )) else {
        // The timer is already running; there's nothing to do here.
        return;
    };

    // Start the timer.
    assert!(
        handler.start().expect("unable to start timer"),
        "timer was already running"
    );
}

/// Get the recent CPU usage, measured as a real value between zero and one.
pub fn current_cpu_usage() -> [f32; 1] {
    // FIXME: Get the second core's CPU usage as well
    let st = &STATE;
    let numerator: u32 = st
        .busy_history
        .iter()
        .map(|x| x.load(atomic::Ordering::Relaxed).count_ones())
        .sum();
    let denominator = st.busy_history.len() as u32 * usize::BITS;
    [numerator as f32 / denominator as f32]
}

struct State {
    cursor: atomic::AtomicUsize,
    busy_history: [atomic::AtomicUsize; 16],
}

const ZERO: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

static STATE: State = State {
    cursor: ZERO,
    // The repeat operand can be `!Copy` if it directly refers to a constant item
    // <https://doc.rust-lang.org/1.62.0/reference/expressions/array-expr.html>
    busy_history: [ZERO; 16],
};

fn timer_handler(cx: CpuCx<'_>) {
    // Check if the timer was taken on a WFI instruction
    let taken_on_wfi = unsafe {
        // If that's indeed the case, `ctx.pc` should point to the next instruction
        let cx = cx.as_raw().as_ref();
        let ptr = cx.pc.wrapping_sub(4);
        ptr % 4 == 0 && solid::abi::SOLID_MEM_IsValid(ptr as _, 4).0 != 0 && {
            let instr = (ptr as *const u32).read_volatile();
            instr == 0xD503207F // WFI
        }
    };

    let st = &STATE;
    let cursor = st.cursor.load(atomic::Ordering::Relaxed);
    st.cursor
        .store(cursor.wrapping_add(1), atomic::Ordering::Relaxed);

    let i = cursor % st.busy_history.len();

    // Record the busyness
    let mut bmp = st.busy_history[i].load(atomic::Ordering::Relaxed);
    bmp = (bmp << 1) | (!taken_on_wfi as usize);
    st.busy_history[i].store(bmp, atomic::Ordering::Relaxed);
}
