//! CPU usage monitor
#![allow(non_snake_case)]
use std::{cell::UnsafeCell, sync::atomic};

mod solid_abi {
    use super::*;

    extern "C" {
        pub fn SOLID_TIMER_RegisterTimer(pHandler: *const SOLID_TIMER_HANDLER) -> i32;
        pub fn SOLID_MEM_IsValid(va: usize, size: usize) -> i32;
    }

    pub const SOLID_TIMER_TYPE_INTERVAL: i32 = 1;

    #[repr(C)]
    pub struct SOLID_TIMER_HANDLER {
        pub pNext: UnsafeCell<*mut Self>,
        pub pCallQ: UnsafeCell<*mut Self>,
        pub globalTick: UnsafeCell<u64>,
        pub r#type: i32,
        pub time: u32,
        pub func: unsafe extern "C" fn(param: *mut (), ctx: *mut SOLID_CPU_CONTEXT),
        pub param: *mut (),
    }

    unsafe impl Sync for SOLID_TIMER_HANDLER {}

    #[repr(C)]
    pub struct SOLID_CPU_CONTEXT {
        pub xarm: [usize; 31],
        pub sp: usize,
        pub pc: usize,
        pub pstate: u32,
        pub spsel: u32,
        pub pNext: *mut Self,
        pub pFPU: *mut (),
    }
}

static TIMER_HANDLER: solid_abi::SOLID_TIMER_HANDLER = solid_abi::SOLID_TIMER_HANDLER {
    pNext: UnsafeCell::new(std::ptr::null_mut()),
    pCallQ: UnsafeCell::new(std::ptr::null_mut()),
    globalTick: UnsafeCell::new(0),
    r#type: solid_abi::SOLID_TIMER_TYPE_INTERVAL,
    time: 1000, // usec
    func: timer_handler,
    param: std::ptr::null_mut(),
};

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

unsafe extern "C" fn timer_handler(_: *mut (), ctx: *mut solid_abi::SOLID_CPU_CONTEXT) {
    // Check if the timer was taken on a WFI instruction
    let taken_on_wfi = unsafe {
        // If that's indeed the case, `ctx.pc` should point to the next instruction
        let ptr = (*ctx).pc.wrapping_sub(4);
        ptr % 4 == 0 && solid_abi::SOLID_MEM_IsValid(ptr, 4) != 0 && {
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

pub fn init() {
    static INITED: atomic::AtomicBool = atomic::AtomicBool::new(false);
    if !INITED.swap(true, atomic::Ordering::Relaxed) {
        let ret = unsafe { solid_abi::SOLID_TIMER_RegisterTimer(&TIMER_HANDLER) };
        assert_eq!(ret, 0);
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
