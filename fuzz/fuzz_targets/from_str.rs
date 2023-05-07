#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str::FromStr;
use euid::euid::EUID;

fuzz_target!(|data: &[u8]| {
    let _s = match String::from_utf8(data.to_vec());
});
