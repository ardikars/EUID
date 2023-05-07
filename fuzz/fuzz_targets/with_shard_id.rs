#![no_main]

use libfuzzer_sys::fuzz_target;
use euid::euid::EUID;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 10 {
        let shard_id = u16::from_be_bytes([data[0], data[1]]);
        let euid = EUID::with_shard_id(shard_id);
        let random_epoch = u64::from_be_bytes([data[5], data[4], data[1], data[3], data[2], data[5], data[0], data[7]]);
        let _ = euid.timestamp_with_epoch(random_epoch);
        let _ = euid.version();
        let _ = euid.timestamp();
        let _ = euid.shard_id();
    }
});
