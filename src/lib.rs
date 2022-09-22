#![cfg_attr(feature = "microram", feature(lang_items))]
#![cfg_attr(feature = "microram", no_std)]

#[cfg(feature = "inline-secrets")] extern crate scuttlebutt_attack_secrets;

pub mod server;
pub mod attacker;
pub mod comm_trace;
#[cfg(feature = "secrets")] pub mod comm_trace_data;
pub mod comm_trace_types;
#[cfg(feature = "secrets")] pub mod io_merged;
pub mod io_kernel;
#[cfg(feature = "secrets")] pub mod kernel;
pub mod util;


#[cfg(feature = "microram")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    extern "C" {
        fn __cc_answer(code: i32) -> !;
    }
    unsafe {
        __cc_answer(0);
    }
}

#[cfg(feature = "microram")]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
