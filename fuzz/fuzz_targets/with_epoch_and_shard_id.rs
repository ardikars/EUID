#![no_main]

use libfuzzer_sys::fuzz_target;
use euid::euid::EUID;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 10 {
        let epoch = u64::from_be_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
        let shard_id = u16::from_be_bytes([data[8], data[9]]);
        let _ = EUID::with_epoch_and_shard_id(epoch, shard_id);
    }
});
