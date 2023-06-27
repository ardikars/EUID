// MIT License
//
// Copyright (c) 2023 Ardika Rommy Sanjaya
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

mod base32;
mod check;
mod random;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidLength(usize, usize),
    InvalidCharacter(char),
    InvalidCheckmod(usize, usize),
}

/// EUID contains two main components header and randomness.
/// The header store information about the ID and user-attached data (extension).
/// Timestamps are included in the header to make EUID sortable,
/// but the order isn't guaranteed if EUID is generated with the same milliseconds.
/// We can provide some guarantee regarding sort order by incrementing randomness (at least significant bit) by 1.
/// In case overflow happens when incrementing randomness, the generation should fail.
///
/// Binary layout (Big Endian):
/// ```text
///        0               1               2               3
/// 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                       32_bit_uint_t_high                      |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// | 10_bit_uint_t_low |   N Bit Random + Ext Data   |Ext Len| Ver |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                       32_bit_uint_random                      |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                       32_bit_uint_random                      |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Default, Copy, Clone, Debug)]
pub struct EUID(u64, u64);

/// A Standard implementation of EUID (the naming convention should follow the language best practice).
impl EUID {
    const EPOCH: u64 = 0x0;

    const VERSION_1: u64 = 0x0;
    const TIMESTAMP_BITMASK: u64 = 0x3ffffffffff;
    const VERSION_BITMASK: u64 = 0x7;
    const EXT_LEN_BITMASK: u64 = 0xf;
    const EXT_DATA_BITMASK: u64 = 0x7fff;

    /// Create random EUID.
    ///
    /// Example:
    /// ```rust
    /// use euid::EUID;
    ///
    /// let euid: EUID = EUID::create().unwrap_or_default();
    /// ```
    pub fn create() -> Option<EUID> {
        let ts: u64 = EUID::get_timestamp_from_epoch(EUID::EPOCH);
        EUID::create_with_timestamp_and_extension(ts, 0)
    }

    /// Create random EUID with attachable data (max 15 bit).
    /// None will returns if the extenstion (user attached data) is more then 15 bits.
    ///
    /// Example:
    /// ```rust
    /// use euid::EUID;
    ///
    /// let euid: Option<EUID> = EUID::create_with_extension(1);
    /// ```
    pub fn create_with_extension(extension: u16) -> Option<EUID> {
        if extension > ((EUID::EXT_DATA_BITMASK) as u16) {
            None
        } else {
            EUID::create_with_timestamp_and_extension(
                EUID::get_timestamp_from_epoch(EUID::EPOCH),
                extension,
            )
        }
    }

    /// Returns EUID version number.
    pub fn version(&self) -> u8 {
        ((self.0 & EUID::VERSION_BITMASK) + 1) as u8
    }

    /// Returns user attached data (extension).
    pub fn extension(&self) -> u16 {
        let ext_len: u64 = (self.0 >> 3) & EUID::EXT_LEN_BITMASK;
        let bitmask: u64 = (1 << ext_len) - 1;
        ((self.0 >> 7) & bitmask) as u16
    }

    /// Returns timestamp in milliseconds.
    pub fn timestamp(&self) -> u64 {
        ((self.0 >> 22) & EUID::TIMESTAMP_BITMASK) + EUID::EPOCH
    }

    /// Derive monotonic EUID.
    ///
    /// This function generate sortable EUID, None returns if overflow happens while incrementing randomness.
    pub fn next(&self) -> Option<EUID> {
        let timestamp: u64 = EUID::get_timestamp_from_epoch(EUID::EPOCH);
        if timestamp == self.timestamp() {
            let a: u64 = (self.1 >> 32) + 1;
            if a > u32::MAX as u64 {
                None
            } else {
                let b: u64 = random::random_u32() as u64;
                let c: u64 = (a << 32) | b;
                Some(EUID(self.0, c))
            }
        } else {
            EUID::create_with_timestamp_and_extension(timestamp, self.extension())
        }
    }

    pub fn encode(&self, checkmod: bool) -> String {
        base32::encode(self, checkmod)
    }

    fn get_ext_bit_len(ext: u16) -> u64 {
        let mut x: u16 = ext & 0x7fff;
        if x == 0 {
            0
        } else {
            let mut n: u64 = 0;
            if x <= 0x00ff {
                n += 8;
                x <<= 8;
            }
            if x <= 0x0fff {
                n += 4;
                x <<= 4;
            }
            if x <= 0x3fff {
                n += 2;
                x <<= 2;
            }
            if x <= 0x7fff {
                n += 1;
            }
            16 - n
        }
    }

    fn normalize_timestamp(now: u64, epoch: u64) -> u64 {
        if epoch < now {
            now - epoch
        } else {
            now
        }
    }

    fn get_timestamp_from_epoch(epoch: u64) -> u64 {
        let duration: Result<std::time::Duration, std::time::SystemTimeError> =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
        match duration {
            Ok(now) => {
                let millis: u64 = now.as_millis() as u64;
                let final_epoch: u64 = epoch & EUID::TIMESTAMP_BITMASK;
                EUID::normalize_timestamp(millis, final_epoch)
            }
            Err(_) => 0,
        }
    }

    #[inline(always)]
    fn create_with_timestamp_and_extension(timestamp: u64, extension: u16) -> Option<EUID> {
        if timestamp > EUID::TIMESTAMP_BITMASK {
            None
        } else {
            let version: u64 = EUID::VERSION_1 & EUID::VERSION_BITMASK;
            let ext_len: u64 = EUID::get_ext_bit_len(extension);
            let ext_data: u64 = (extension as u64) & EUID::EXT_DATA_BITMASK;
            let remain_rand: u64 = (random::random_u32() & ((1 << (15 - ext_len)) - 1)) as u64;
            let hi: u64 = (timestamp << 22)
                | (remain_rand << (7 + ext_len))
                | (ext_data << 7)
                | ext_len << 3
                | version;
            Some(EUID(hi, random::random_u64()))
        }
    }
}

impl std::fmt::Display for EUID {
    /// Encode to lexicographically sortable string.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.encode(true))
    }
}

impl From<EUID> for u128 {
    fn from(val: EUID) -> Self {
        ((val.0 as u128) << 64) | (val.1 as u128)
    }
}

impl From<u128> for EUID {
    fn from(value: u128) -> EUID {
        let hi: u64 = (value >> 64) as u64;
        let lo: u64 = (value & 0xffffffffffffffff) as u64;
        EUID(hi, lo)
    }
}

impl std::str::FromStr for EUID {
    type Err = Error;

    /// Parse string representation of EUID.
    fn from_str(encoded: &str) -> Result<EUID, Self::Err> {
        match base32::decode(encoded) {
            Ok(euid) => Ok(euid),
            Err(e) => Err(e),
        }
    }
}

impl PartialEq for EUID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for EUID {}

impl Ord for EUID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for EUID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0 != other.0 {
            return Some(if self.0 > other.0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            });
        }
        if self.1 != other.1 {
            return Some(if self.1 > other.1 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            });
        }
        Some(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {

    use std::{
        str::FromStr,
        time::{SystemTime, UNIX_EPOCH},
    };

    use rand::{seq::SliceRandom, thread_rng};

    use crate::{random, Error, EUID};

    fn get_timestamp_diff(start: u64, timestamp: u64) -> u64 {
        if start < timestamp {
            timestamp - start
        } else {
            start - timestamp
        }
    }

    fn get_ext_bit_len0(v: u16) -> u64 {
        let mut i: i32 = 14;
        while i >= 0 {
            if (v >> i) != 0 {
                return (i as u64) + 1;
            }
            i -= 1;
        }
        return 0;
    }

    #[test]
    fn normalize_timestamp_test() {
        assert_eq!(1, EUID::normalize_timestamp(1, 2));
        assert_eq!(1, EUID::normalize_timestamp(2, 1));
    }

    #[test]
    fn get_timestamp_diff_test() {
        assert_eq!(1, get_timestamp_diff(1, 2));
        assert_eq!(1, get_timestamp_diff(2, 1));
    }

    #[test]
    fn create_with_timestamp_and_extension_test() {
        assert_eq!(None, EUID::create_with_timestamp_and_extension(u64::MAX, 0));
        for i in 0u32..65535 {
            let epoch: u64 = random::random_u32() as u64;
            let ts: u64 = EUID::get_timestamp_from_epoch(epoch);
            let now: u64 = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let euid: EUID = EUID::create_with_timestamp_and_extension(ts, i as u16).unwrap();
            let timestamp: u64 = euid.timestamp();
            assert!(timestamp <= EUID::TIMESTAMP_BITMASK);
            let t: u64 = now - epoch;
            let diff: u64 = get_timestamp_diff(t, timestamp);
            assert!(diff < 50);
            assert_eq!((i as u64 & EUID::EXT_DATA_BITMASK) as u16, euid.extension());
            assert!(euid.extension() as u64 <= EUID::EXT_DATA_BITMASK);
            assert!(EUID::get_ext_bit_len(euid.extension()) <= EUID::EXT_LEN_BITMASK);
            assert_eq!(1, euid.version());
            assert!((euid.version() - 1) as u64 <= EUID::VERSION_BITMASK);
        }
    }

    #[test]
    fn get_ext_bit_len_test() {
        let max: u16 = 1 << 15;
        for i in 0..max {
            assert_eq!(EUID::get_ext_bit_len(i), get_ext_bit_len0(i));
        }
    }

    #[test]
    fn create_test() {
        let now: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let euid: EUID = EUID::create().unwrap_or_default();
        let timestamp: u64 = euid.timestamp();
        let diff: u64 = get_timestamp_diff(now, timestamp);
        assert!(diff < 50);
        assert_eq!(0, euid.extension());
        assert_eq!(1, euid.version());
    }

    #[test]
    fn create_with_extension_test() {
        for i in 0u32..0x7fff {
            let now: u64 = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let euid: EUID = EUID::create_with_extension(i as u16).unwrap();
            let timestamp: u64 = euid.timestamp();
            let diff: u64 = get_timestamp_diff(now, timestamp);
            assert!(diff < 50);
            assert_eq!((i as u16 & 0x7fff), euid.extension());
            assert_eq!(1, euid.version());
        }
        assert_eq!(None, EUID::create_with_extension(0x7fff + 1));
    }

    #[test]
    fn conversion_test() {
        for i in 0..0x7fff {
            let euid: EUID = EUID::create_with_extension(i as u16).unwrap();
            let encoded: String = euid.to_string();
            assert_eq!(27, encoded.len());
            let decoded: EUID = EUID::from_str(&encoded).unwrap();
            assert_eq!(euid, decoded);
            let e128: u128 = euid.into();
            assert_eq!(EUID::from(e128), euid);
            assert_eq!(std::cmp::Ordering::Equal, EUID::from(e128).cmp(&euid));
            assert_eq!(euid, euid.clone());
            let euid_copy: EUID = euid;
            assert_eq!(euid, euid_copy);
            let euidx: EUID = e128.into();
            assert_eq!(euid, euidx);
            let x: u128 = u128::from(euid);
            assert_eq!(x, e128);
        }
        assert_eq!(None, EUID::create_with_extension(0x7fff + 1));
        assert_eq!(
            Err(Error::InvalidLength(25, 27)),
            EUID::from_str("C8754X9NN8H80X298KRKERG8K")
        );
        assert_eq!(
            Err(Error::InvalidLength(28, 27)),
            EUID::from_str("C8754X9NN8H80X298KRKERG8K888")
        );
        assert_eq!(
            Err(Error::InvalidCharacter('U')),
            EUID::from_str("C8754X9NN8H80X298KRKERG8KU8")
        );
    }

    #[test]
    fn monotonic_test() {
        let hi: u64 = EUID::create().unwrap().0;
        let euid: EUID = EUID(hi, u64::MAX);
        assert_eq!(None, euid.next());

        let mut euids: Vec<EUID> = Vec::<EUID>::new();
        for i in 0usize..0x7fff {
            if i == 0 {
                euids.push(EUID::create_with_extension(i as u16).unwrap());
            } else {
                euids.push(euids[i - 1].next().unwrap())
            }
        }
        assert_eq!(None, EUID::create_with_extension(0x7fff + 1));
        let mut unordered: Vec<EUID> = euids.clone();
        unordered.shuffle(&mut thread_rng());
        let mut ordered: Vec<EUID> = unordered.clone();
        ordered.sort();
        for i in 0..euids.len() {
            assert_eq!(euids[i], ordered[i]);
            assert_eq!(euids[i].to_string(), ordered[i].to_string());
        }
    }

    #[test]
    fn print_test() {
        let euid: EUID = EUID::create().unwrap_or_default();
        println!("{}", euid.to_string());
        println!("{:?}", euid.to_string());
    }
}
