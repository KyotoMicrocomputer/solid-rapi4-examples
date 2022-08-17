//! CPU usage monitor
use solid::{thread::CpuCx, timer};
use std::{pin::Pin, sync::atomic};
use takecell::TakeCell;

/// The timer to measure CPU activity.
static TIMER: TakeCell<timer::Timer<fn(CpuCx<'_>)>> = TakeCell::new(timer::Timer::new(
    timer::Schedule::Interval(timer::Usecs32(1_000)),
    timer_handler,
));

pub fn init() {
    // Get a mutable reference to the `Timer` object. Each instance of `TakeCell` allows us to do
    // this *only once*. In exchange, we can get `&'static mut Timer`.
    if let Some(handler) = TIMER.take() {
        // Convert `&'static mut Timer` to `Pin<&'static mut Timer>` (a pinned mutable reference).
        //
        // `&'static mut Timer` can be safely interpreted as a pinned reference `Pin<&'static mut
        // Timer>`. You need a pinned reference to call `Timer::start` because you must keep the
        // contained `SOLID_TIMER_HANDLER` alive while it's linked to the system timer list.
        let handler = Pin::static_mut(handler);

        // Start the timer.
        assert!(
            handler.start().expect("unable to start timer"),
            "timer was already running"
        );
    }
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

    let i = (cursor / usize::BITS as usize) % st.busy_history.len();
    let mask = 1usize.rotate_left(cursor as u32);

    // Record the busyness
    let mut bmp = st.busy_history[i].load(atomic::Ordering::Relaxed);
    if taken_on_wfi {
        bmp &= !mask;
    } else {
        bmp |= mask;
    }
    st.busy_history[i].store(bmp, atomic::Ordering::Relaxed);
}
