#![no_std]

// SAFETY:
// Only one symbol with this name exists.
#[unsafe(no_mangle)]
pub fn start() -> i32 {
    42
}

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
