#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)] // google/autocxx#578
use autocxx::prelude::*;
use core::arch::asm;

include_cpp! {
    #include "src/abi.hpp"

    extern_cpp_opaque_type!("SOLID_TIMER_HANDLER", super::SOLID_TIMER_HANDLER)

    generate!("SOLID_TIMER_TYPE_ONESHOT")
    generate!("SOLID_TIMER_TYPE_INTERVAL")
    generate!("SOLID_TIMER_TYPE_GLOBALTICK")
    generate!("SOLID_TIMER_GetCurrentTick")
    generate!("SOLID_TIMER_ToUsec")
    generate!("SOLID_TIMER_RegisterTimer")
    generate!("SOLID_TIMER_UnRegisterTimer")
    generate!("SOLID_TIMER_WaitNsec")
    generate!("SOLID_TIMER_GetTicksPerSec")
    generate!("SOLID_TIMER_GetMaxTicks")
    generate!("SOLID_TIMER_GetMaxTimerTime")
    generate!("SOLID_TIMER_Suspend")
    generate!("SOLID_TIMER_Resume")

    // TODO: Incorrect definition of SOLID_SMP_GetCpuId due to #3310
    // generate!("SOLID_SMP_GetCpuId")
    generate!("SOLID_SMP_ForEachCpu")
    generate!("SOLID_SMP_RequestExec")
    generate!("SOLID_SMP_SetRegister")
    generate!("SOLID_SMP_SetJump")

    // TODO: Make these `pub(crate)`
    generate!("DEFINED_SOLID_TIMER_EACHCPU")
    generate!("SOLID_TIMER_HANDLER_OFFSET0")
    generate!("SOLID_TIMER_HANDLER_OFFSET1")
    generate!("SOLID_TIMER_HANDLER_OFFSET2")
    generate!("SOLID_TIMER_HANDLER_OFFSET3")
    generate!("SOLID_TIMER_HANDLER_OFFSET4")
    generate!("SOLID_TIMER_HANDLER_OFFSET5")
    generate!("SOLID_TIMER_HANDLER_OFFSET6")

    generate!("SOLID_CPU_CONTEXT")
    generate!("SOLID_REGISTER")
}

#[cxx::bridge]
mod ffi2 {
    // TODO: Some fields of this struct have intentionally different types to
    // circumvent the current restrictions of `cxx`, which might be alleviated
    // later. This struct might not be ready to be `pub`.
    pub struct SOLID_TIMER_HANDLER {
        /// Used by the system.
        pub pNext: *mut SOLID_TIMER_HANDLER,
        /// Used by the system.
        pub pCallQ: *mut SOLID_TIMER_HANDLER,
        /// If `ty == SOLID_TIMER_TYPE_GLOBALTICK`, specifies the absolute
        /// expiration time. Otherwise, used by the system.
        pub globalTick: u64,
        /// The type of the timer.
        pub ty: u32,
        /// The timer period, measured in microseconds.
        pub time: u32,
        /// `unsafe extern "C" fn(param: Cx, ctx: Cx)`
        pub func: *mut u8,
        /// `Cx`
        pub param: *mut u8,
    }
}

pub use self::{ffi::*, ffi2::*};
pub use autocxx::c_int;

extern "C" {
    // TODO: Manually definition is necessary due to #3310
    pub fn SOLID_SMP_GetCpuId() -> c_int;
}

// TODO: Do we really want to keep using `autocxx::c_int`? `autocxx` doesn't
//       use `c_int` for `#define`s, btw.
pub const SOLID_ERR_OK: c_int = c_int(0);
pub const SOLID_ERR_PAR: c_int = c_int(-17);
pub const SOLID_ERR_MACV: c_int = c_int(-26);
pub const SOLID_ERR_NOMEM: c_int = c_int(-33);
pub const SOLID_ERR_NORES: c_int = c_int(-35);
pub const SOLID_ERR_NOTFOUND: c_int = c_int(-1000);
pub const SOLID_ERR_NOTSUPPORTED: c_int = c_int(-1001);
pub const SOLID_ERR_EBADF: c_int = c_int(-1002);
pub const SOLID_ERR_INVALIDCONTENT: c_int = c_int(-1003);
pub const SOLID_ERR_NOTUSED: c_int = c_int(-1004);
pub const SOLID_ERR_ALREADYUSED: c_int = c_int(-1005);
pub const SOLID_ERR_OUTOFBOUND: c_int = c_int(-1006);
pub const SOLID_ERR_BADSEQUENCE: c_int = c_int(-1007);
pub const SOLID_ERR_UNKNOWNDEVICE: c_int = c_int(-1008);
pub const SOLID_ERR_BUSY: c_int = c_int(-1009);
pub const SOLID_ERR_TIMEOUT: c_int = c_int(-1010);
pub const SOLID_ERR_INVALIDACCESS: c_int = c_int(-1011);
pub const SOLID_ERR_NOTREADY: c_int = c_int(-1012);

#[inline]
pub unsafe fn SOLID_MUTEX_PushInt() -> SOLID_REGISTER {
    let status;
    match () {
        #[cfg(target_arch = "aarch64")]
        () => asm!(
            "mrs {}, DAIF
            msr DAIFset, #3",
            out(reg) status,
        ),
    }
    status
}

#[inline]
pub unsafe fn SOLID_MUTEX_PopInt(status: SOLID_REGISTER) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => match status & 0xc0 {
            0x00 => asm!("msr DAIFclr,#3"),
            0x40 => asm!("msr DAIFclr,#2"),
            0x80 => asm!("msr DAIFclr,#1"),
            _ => {}
        },
    }
}
