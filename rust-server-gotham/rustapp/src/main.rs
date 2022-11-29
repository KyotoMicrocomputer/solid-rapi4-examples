//! Entry point for host build (unused in SOLID-Rust)
use rustapp as _;

extern "C" {
    fn slo_main();
}

fn main() {
    unsafe { slo_main() };
}
