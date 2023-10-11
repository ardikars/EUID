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

//! # EUID
//! Reference implementation of EUID.
//!

mod base32;
mod check;
mod random;
mod time;

/// Error enum.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// EUID must have 27 character in size.
    InvalidLength(usize, usize),
    /// EUID use a set of 10 digits and 22 letters, excluding 4 of the 26 letters: I L O U.
    InvalidCharacter(char),
    /// Invalid entry (typo).
    InvalidCheckmod(usize, usize),
}

/// Extendable Universally Unique Identifier or EUID contains two main components:
/// header and random number.
///
/// Binary layout (Big Endian):
/// ```text
///        0               1               2               3
/// 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Timestamp High                        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |      Timestamp Low      | N Bit Random + Ops Ext Data |Ext Len|
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             Random                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             Random                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Default, Copy, Clone, Debug)]
pub struct EUID(u64, u64);

/// A Standard implementation of EUID.
impl EUID {
    const TIMESTAMP_BITMASK: u64 = 0x1fffffffffff;
    const EXT_LEN_BITMASK: u64 = 0xf;
    const EXT_DATA_BITMASK: u64 = 0x7fff;

    /// Create random EUID.
    /// None will returns if the EUID is created after Friday, December 12, 3084 12:41:28.831 PM (UTC).
    ///
    /// Example:
    /// ```rust
    /// use euid::EUID;
    ///
    /// let euid: EUID = EUID::create().unwrap_or_default();
    /// println!("{}", euid); // with check-mod.
    /// println!("{}", euid.encode(true)); // with check-mod.
    /// println!("{}", euid.encode(false)); // without check-mod.
    /// ```
    pub fn create() -> Option<EUID> {
        EUID::create_with_timestamp(time::current_timestamp())
    }

    /// Create random EUID with attachable data (max 15 bit).
    /// None will returns if the EUID is created after Friday, December 12, 3084 12:41:28.831 PM (UTC)
    /// or the extenstion (user attached data) is more then 15 bits.
    ///
    /// Example:
    /// ```rust
    /// use euid::EUID;
    ///
    /// let euid: EUID = EUID::create_with_extension(1).unwrap_or_default();
    /// println!("{}", euid); // with check-mod.
    /// println!("{}", euid.encode(true)); // with check-mod.
    /// println!("{}", euid.encode(false)); // without check-mod.
    ///
    /// let overflowed_euid: Option<EUID> = EUID::create_with_extension(32768);
    /// assert_eq!(None, overflowed_euid);
    /// ```
    pub fn create_with_extension(extension: u16) -> Option<EUID> {
        if extension > ((EUID::EXT_DATA_BITMASK) as u16) {
            None
        } else {
            EUID::create_with_timestamp_and_extension(time::current_timestamp(), extension)
        }
    }

    /// Returns user attached data (extension), or None if no attached data.
    pub fn extension(&self) -> Option<u16> {
        let ext_len: u64 = self.0 & EUID::EXT_LEN_BITMASK;
        if ext_len == 0 {
            None
        } else {
            let bitmask: u64 = (1 << ext_len) - 1;
            Some(((self.0 >> 4) & bitmask) as u16)
        }
    }

    /// Returns timestamp in milliseconds.
    pub fn timestamp(&self) -> u64 {
        (self.0 >> 19) & EUID::TIMESTAMP_BITMASK
    }

    /// Derive monotonic EUID.
    ///
    /// This function generate sortable EUID, None returns if overflow happens when incrementing randomness.
    pub fn next(&self) -> Option<EUID> {
        let timestamp: u64 = time::current_timestamp();
        if timestamp == self.timestamp() {
            let r_hi = self.1 >> 32;
            if r_hi == 0xffffffff {
                None
            } else {
                Some(EUID(
                    self.0,
                    ((r_hi + 1) << 32) | random::random_u32() as u64,
                ))
            }
        } else {
            match self.extension() {
                Some(ext) => EUID::create_with_timestamp_and_extension(timestamp, ext),
                None => EUID::create_with_timestamp(timestamp),
            }
        }
    }

    /// Encode EUID to string Base-32 string.
    ///
    /// Example:
    /// ```rust
    /// use euid::EUID;
    ///
    /// let euid: EUID = EUID::create().unwrap_or_default();
    /// println!("{}", euid.encode(true)); // with check-mod.
    /// println!("{}", euid.encode(false)); // without check-mod.
    /// ```
    pub fn encode(&self, checkmod: bool) -> String {
        base32::encode(self, checkmod)
    }

    #[inline(always)]
    fn get_ext_bit_len(ext: u16) -> u64 {
        let mut x: u16 = ext & 0x7fff;
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

    #[inline(always)]
    fn create_with_timestamp(timestamp: u64) -> Option<EUID> {
        if timestamp > EUID::TIMESTAMP_BITMASK {
            None
        } else {
            let (r0, r1) = random::random_u128();
            Some(EUID((timestamp << 19) | ((r0 & 0x7fff) << 4), r1))
        }
    }

    #[inline(always)]
    fn create_with_timestamp_and_extension(timestamp: u64, extension: u16) -> Option<EUID> {
        if timestamp > EUID::TIMESTAMP_BITMASK {
            None
        } else {
            let ext_data: u64 = extension as u64;
            if ext_data > EUID::EXT_DATA_BITMASK {
                None
            } else {
                let ext_len: u64 = EUID::get_ext_bit_len(extension);
                let (r0, r1) = random::random_u128();
                let remain_rand: u64 = r0 & ((1 << (15 - ext_len)) - 1);
                let hi: u64 =
                    (timestamp << 19) | (remain_rand << (4 + ext_len)) | (ext_data << 4) | ext_len;
                Some(EUID(hi, r1))
            }
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

impl std::hash::Hash for EUID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl Ord for EUID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<[u8; 16]> for EUID {
    fn from(value: [u8; 16]) -> Self {
        let id: u128 = u128::from_be_bytes(value);
        EUID((id >> 64) as u64, (id & 0xffffffffffffffff) as u64)
    }
}

impl From<EUID> for [u8; 16] {
    #[cfg(not(feature = "euid_64"))]
    fn from(value: EUID) -> Self {
        (((value.0 as u128) << 64) | (value.1 as u128)).to_be_bytes()
    }

    #[cfg(feature = "euid_64")]
    fn from(value: EUID) -> Self {
        let mut v: [u8; 16] = [0u8; 16];
        v[0] = ((value.0 >> 56) & 0xff) as u8;
        v[1] = ((value.0 >> 48) & 0xff) as u8;
        v[2] = ((value.0 >> 40) & 0xff) as u8;
        v[3] = ((value.0 >> 32) & 0xff) as u8;
        v[4] = ((value.0 >> 24) & 0xff) as u8;
        v[5] = ((value.0 >> 16) & 0xff) as u8;
        v[6] = ((value.0 >> 8) & 0xff) as u8;
        v[7] = (value.0 & 0xff) as u8;
        v[8] = ((value.0 >> 56) & 0xff) as u8;
        v[9] = ((value.0 >> 48) & 0xff) as u8;
        v[10] = ((value.0 >> 40) & 0xff) as u8;
        v[11] = ((value.0 >> 32) & 0xff) as u8;
        v[12] = ((value.0 >> 24) & 0xff) as u8;
        v[13] = ((value.0 >> 16) & 0xff) as u8;
        v[14] = ((value.0 >> 8) & 0xff) as u8;
        v[15] = (value.0 & 0xff) as u8;
        v
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

    use std::hash::Hasher;
    use std::str::FromStr;

    use rand::{seq::SliceRandom, thread_rng};

    fn get_timestamp_diff(start: u64, timestamp: u64) -> u64 {
        if start < timestamp {
            timestamp - start
        } else {
            start - timestamp
        }
    }

    fn get_ext_bit_len0(v: u16) -> u64 {
        let mut i: i32 = 14;
        while i > 0 {
            if (v >> i) != 0 {
                return (i as u64) + 1;
            }
            i -= 1;
        }
        1
    }

    #[test]
    fn get_timestamp_diff_test() {
        assert_eq!(1, get_timestamp_diff(1, 2));
        assert_eq!(1, get_timestamp_diff(2, 1));
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
                let final_epoch: u64 = epoch & 0x3ffffffffff;
                normalize_timestamp(millis, final_epoch)
            }
            Err(_) => 0,
        }
    }

    #[test]
    fn create_with_timestamp_and_extension_test() {
        assert_eq!(
            None,
            crate::EUID::create_with_timestamp_and_extension(u64::MAX, 0)
        );
        assert_eq!(
            None,
            crate::EUID::create_with_timestamp_and_extension(
                crate::time::current_timestamp(),
                (crate::EUID::EXT_DATA_BITMASK + 1) as u16
            )
        );
        for i in 0u32..65535 {
            let epoch: u64 = crate::random::random_u32() as u64;
            let ts: u64 = get_timestamp_from_epoch(epoch);
            let now: u64 = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let euid = crate::EUID::create_with_timestamp_and_extension(
                ts,
                ((i as u64) & crate::EUID::EXT_DATA_BITMASK) as u16,
            )
            .unwrap();
            let timestamp: u64 = euid.timestamp();
            assert!(timestamp <= crate::EUID::TIMESTAMP_BITMASK);
            let t: u64 = now - epoch;
            let diff: u64 = get_timestamp_diff(t, timestamp);
            assert!(diff < 50);
            assert_eq!(
                (i as u64 & crate::EUID::EXT_DATA_BITMASK) as u16,
                euid.extension().unwrap()
            );
            assert!(euid.extension().unwrap() as u64 <= crate::EUID::EXT_DATA_BITMASK);
            assert!(
                crate::EUID::get_ext_bit_len(euid.extension().unwrap())
                    <= crate::EUID::EXT_LEN_BITMASK
            );
        }
    }

    #[test]
    fn get_ext_bit_len_test() {
        let max: u16 = 1 << 15;
        for i in 0..max {
            assert_eq!(crate::EUID::get_ext_bit_len(i), get_ext_bit_len0(i));
        }
    }

    #[test]
    fn create_test() {
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let euid: crate::EUID = crate::EUID::create().unwrap_or_default();
        let timestamp: u64 = euid.timestamp();
        let diff: u64 = get_timestamp_diff(now, timestamp);
        assert!(diff < 50);
        assert_eq!(None, euid.extension());
    }

    #[test]
    fn create_with_extension_test() {
        for i in 0u64..crate::EUID::EXT_DATA_BITMASK {
            let now: u64 = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let euid: crate::EUID = crate::EUID::create_with_extension(i as u16).unwrap();
            let timestamp: u64 = euid.timestamp();
            let diff: u64 = get_timestamp_diff(now, timestamp);
            assert!(diff < 50);
            assert_eq!(i, euid.extension().unwrap() as u64);
        }
        assert_eq!(
            None,
            crate::EUID::create_with_extension(crate::EUID::EXT_DATA_BITMASK as u16 + 1)
        );
    }

    #[test]
    fn conversion_test() {
        for i in 0..crate::EUID::EXT_DATA_BITMASK {
            let euid: crate::EUID = crate::EUID::create_with_extension(i as u16).unwrap();
            let encoded: String = euid.to_string();
            assert_eq!(27, encoded.len());
            let decoded: crate::EUID = crate::EUID::from_str(&encoded).unwrap();
            assert_eq!(euid, decoded);
            let e128: u128 = euid.into();
            assert_eq!(crate::EUID::from(e128), euid);
            assert_eq!(
                std::cmp::Ordering::Equal,
                crate::EUID::from(e128).cmp(&euid)
            );
            assert_eq!(euid, euid.clone());
            let euid_copy: crate::EUID = euid;
            assert_eq!(euid, euid_copy);
            let euidx: crate::EUID = e128.into();
            assert_eq!(euid, euidx);
            let x: u128 = u128::from(euid);
            assert_eq!(x, e128);
        }
        assert_eq!(None, crate::EUID::create_with_extension(0x7fff + 1));
        assert_eq!(
            Err(crate::Error::InvalidLength(25, 27)),
            crate::EUID::from_str("C8754X9NN8H80X298KRKERG8K")
        );
        assert_eq!(
            Err(crate::Error::InvalidLength(28, 27)),
            crate::EUID::from_str("C8754X9NN8H80X298KRKERG8K888")
        );
        assert_eq!(
            Err(crate::Error::InvalidCharacter('U')),
            crate::EUID::from_str("C8754X9NN8H80X298KRKERG8KU8")
        );
    }

    #[test]
    fn monotonic_test() {
        let hi: u64 = crate::EUID::create().unwrap().0;
        let euid: crate::EUID = crate::EUID(hi, u64::MAX);
        assert_eq!(None, euid.next());

        let mut euids: Vec<crate::EUID> = Vec::<crate::EUID>::new();
        for i in 0usize..0x7fff {
            if i == 0 {
                euids.push(crate::EUID::create_with_extension(i as u16).unwrap());
            } else {
                euids.push(euids[i - 1].next().unwrap())
            }
        }
        assert_eq!(None, crate::EUID::create_with_extension(0x7fff + 1));
        let mut unordered: Vec<crate::EUID> = euids.clone();
        unordered.shuffle(&mut thread_rng());
        let mut ordered: Vec<crate::EUID> = unordered.clone();
        ordered.sort();
        for i in 0..euids.len() {
            assert_eq!(euids[i], ordered[i]);
            assert_eq!(euids[i].to_string(), ordered[i].to_string());
        }
    }

    #[test]
    fn bytes_test() {
        let euid: crate::EUID = crate::EUID::create().unwrap_or_default();
        let bytes: [u8; 16] = From::from(euid);
        let from_bytes: crate::EUID = From::from(bytes);
        assert_eq!(16, bytes.len());
        assert_eq!(euid, from_bytes);
    }

    #[test]
    fn hash_test() {
        let euid: crate::EUID = crate::EUID::create().unwrap_or_default();
        let mut default_hasher0 = std::collections::hash_map::DefaultHasher::new();
        let mut default_hasher1 = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&euid, &mut default_hasher0);
        std::hash::Hash::hash_slice(&[euid], &mut default_hasher1);
        assert_eq!(default_hasher0.finish(), default_hasher1.finish());
    }

    #[test]
    fn print_test() {
        let euid: crate::EUID = crate::EUID::create().unwrap_or_default();
        println!("{:?}\n{}", euid, euid);
    }
}
