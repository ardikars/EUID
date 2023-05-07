#![no_main]

use libfuzzer_sys::fuzz_target;
use euid::euid::EUID;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 2 {
        let shard_id = u16::from_be_bytes([data[0], data[1]]);
        let _ = EUID::with_shard_id(shard_id);
    }
});
