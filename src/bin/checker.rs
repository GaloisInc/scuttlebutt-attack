#![cfg_attr(feature = "microram", no_std)]
#![cfg_attr(feature = "microram", no_main)]
use scuttlebutt_attack::comm_trace_data;

#[cfg_attr(feature = "microram", no_mangle)]
pub fn main() {
    comm_trace_data::check_trace();
}
