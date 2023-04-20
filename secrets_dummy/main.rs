#![cfg_attr(feature = "microram", no_std)]
#![cfg_attr(feature = "microram", no_main)]
#![cfg_attr(feature = "microram", feature(lang_items))]
extern crate scuttlebutt_attack_secrets_dummy;

fn main() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    extern "C" {
        fn __cc_answer(code: i32) -> !;
    }
    unsafe {
        __cc_answer(0);
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
