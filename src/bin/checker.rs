#![cfg_attr(feature = "microram", no_std)]
#![cfg_attr(feature = "microram", no_main)]
use scuttlebutt_attack::comm_trace_data;
use scuttlebutt_attack::io_merged;

#[cfg_attr(feature = "microram", no_mangle)]
pub fn main() {
    comm_trace_data::check_trace();
    io_merged::exit();
}
