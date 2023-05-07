// MIT License
// 
// Copyright (c) [2023] [Ardika Rommy Sanjaya]
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use std::{time::{UNIX_EPOCH, SystemTime}, fmt::{Display}};

use bech32::ToBase32;

use crate::random;

pub struct EUID([u32; 4]);

// 0                   1                   2                   3
//  0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                       32_bit_uint_t_high                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// | 10_bit_uint_t_low |   N Bit Random + Shard ID   |SID Len| Ver |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                       32_bit_uint_random                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                       32_bit_uint_random                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
impl EUID {

    const TIMESTAMP_BITMASK: u64 = 0x3ffffffffff;
    const SHARD_ID_LEN_BITMASK: u32 = 0xf;
    const VERSION_BITMASK: u32 = 0x7;

    const VERSION: u8 = 0u8;

    fn shard_id_bit_len(v: &u16) -> u32 {
        let mut i: i8 = 14i8;
        while i >= 0 {
            if (v >> i) != 0 {
                return (i + 1) as u32
            }
            i -= 1;
        }
        0 as u32
    }

    fn engine_random_euid(euid: &mut [u32; 4], now: u64, shard_id: u16, version: u8) {
        let shard_id_bit_len = EUID::shard_id_bit_len(&shard_id);
        let w1 = (((now & 0x3ff) as u32) << 22)
                | ((shard_id as u32) << 7)
                | (shard_id_bit_len << 3)
                | version as u32;
        let rw1 = (euid[1] >> 7) & (1 << shard_id_bit_len) << (shard_id_bit_len + 7);
        euid[0] = (now >> 10) as u32;
        euid[1] = w1 | rw1;
    }

    // Using unix epoch and no shard_id
    pub fn random() -> Self {
        let mut euid: [u32; 4] = [0u32; 4];
        random::random(&mut euid);
        let now: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64 & EUID::TIMESTAMP_BITMASK;
        EUID::engine_random_euid(&mut euid, now, 0, EUID::VERSION);
        Self(euid)
    }

    // Using unix epoch and shard_id
    pub fn with_shard_id(shard_id: u16) -> Self {
        let mut euid: [u32; 4] = [0u32; 4];
        random::random(&mut euid);
        let now: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64 & EUID::TIMESTAMP_BITMASK;
        EUID::engine_random_euid(&mut euid, now, shard_id, EUID::VERSION);
        Self(euid)
    }

    // custom epoch and no sharding
    pub fn with_epoch(epoch: u64) -> Self {
        let mut euid: [u32; 4] = [0u32; 4];
        random::random(&mut euid);
        let now: u64 = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64 & EUID::TIMESTAMP_BITMASK) - epoch;
        EUID::engine_random_euid(&mut euid, now, 0, EUID::VERSION);
        Self(euid)
    }

    // custom epoch and using shard_id
    pub fn with_epoch_and_shard_id(epoch: u64, shard_id: u16) -> Self {
        let mut euid: [u32; 4] = [0u32; 4];
        random::random(&mut euid);
        let now: u64 = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64 & EUID::TIMESTAMP_BITMASK) - epoch;
        EUID::engine_random_euid(&mut euid, now, shard_id, EUID::VERSION);
        Self(euid)
    }

    pub fn timestamp(&self) -> u64 {
        ((self.0[0] as u64) << 10) | ((self.0[1] as u64) >> 22)
    }

    pub fn shard_id(&self) -> u16 {
        let shard_id_bit_len = (self.0[1] >> 3) & EUID::SHARD_ID_LEN_BITMASK;
        ((self.0[1] >> 7) & ((1 << shard_id_bit_len) - 1)) as u16
    }

    pub fn version(&self) -> u8 {
        ((self.0[1] & EUID::VERSION_BITMASK) + 1) as u8
    }
    
    pub fn to_string(&self, prefix: &str) -> Result<String, ()> {
        let mut bytes: [u8; 16] = [0u8; 16];
        let mut idx = 0;
        for i in 0..4 {
            let iv = self.0[i].to_be_bytes();
            bytes[idx] = iv[0];
            bytes[idx + 1] = iv[1];
            bytes[idx + 2] = iv[2];
            bytes[idx + 3] = iv[3];
            idx += 4;
        }
        let v: Vec<u8> = Vec::from(bytes);
        return match bech32::encode(prefix, v.to_base32(), bech32::Variant::Bech32m) {
            Ok(encoded) => {
                Ok(encoded)
            },
            Err(_) => {
                Err(())
            }
        }
    }

}

impl Display for EUID {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self.to_string("uid") {
            Ok(encoded) => {
                write!(f, "[version({}), shard_id({}), timestamp({}), encoded({})]",
                        self.version(),
                        self.shard_id(),
                        self.timestamp(),
                        encoded
                    )
            },
            Err(_) => {
                write!(f, "")
            }
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_euid() {
        let result = EUID::random();
        match result.to_string("uid") {
            Ok(encoded) => {
                assert_ne!(String::from(""), encoded);
            },
            Err(_) => {
                assert_eq!("", "");
            }
        }
    }

    #[test]
    fn timestamp() {
        for i in 0..65536 {
            let now = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64 & EUID::TIMESTAMP_BITMASK) ^ (i as u64);
            let mut euid = [0u32; 4];
            EUID::engine_random_euid(&mut euid, now, 0, 0);
            let result = EUID(euid);
            assert_eq!(now, result.timestamp());
        }
    }

    #[test]
    fn shard_id() {
        for i in 0..32768 {
            let result = EUID::with_shard_id(i as u16);
            assert_eq!(i, result.shard_id() as i32);
        }
    }

    #[test]
    fn version() {
        for i in 0..8 {
            let x = i as u8;
            let mut euid = [0u32; 4];
            EUID::engine_random_euid(&mut euid, 0, 0, x);
            let result = EUID(euid);
            assert_eq!(x + 1, result.version() as u8);
        }
    }
}
