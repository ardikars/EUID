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

use crate::{Error, EUID};

#[allow(non_camel_case_types)]
type u5 = usize;

static ENCODING_SYMBOLS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', //
    '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', //
    'G', 'H', 'J', 'K', 'M', 'N', 'P', 'Q', //
    'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z', //
];

const MCP: usize = usize::MAX;

static DECODING_SYMBOLS: &[u5] = &[
    MCP, MCP, MCP, MCP, MCP, MCP, MCP, MCP, // 0
    MCP, MCP, MCP, MCP, MCP, MCP, MCP, MCP, // 8
    MCP, MCP, MCP, MCP, MCP, MCP, MCP, MCP, // 16
    MCP, MCP, MCP, MCP, MCP, MCP, MCP, MCP, // 24
    MCP, MCP, MCP, MCP, MCP, MCP, MCP, MCP, // 32
    MCP, MCP, MCP, MCP, MCP, MCP, MCP, MCP, // 40
    0, 1, 2, 3, 4, 5, 6, 7, // 48
    8, 9, MCP, MCP, MCP, MCP, MCP, MCP, // 56
    MCP, 10, 11, 12, 13, 14, 15, 16, // 64
    17, 1, 18, 19, 1, 20, 21, 0, // 72
    22, 23, 24, 25, 26, MCP, 27, 28, // 80
    29, 30, 31, MCP, MCP, MCP, MCP, MCP, // 88
    MCP, 10, 11, 12, 13, 14, 15, 16, // 96
    17, 1, 18, 19, 1, 20, 21, 0, // 104
    22, 23, 24, 25, 26, MCP, 27, 28, // 112
    29, 30, 31, // 120
];

#[cfg(not(feature = "non_binary"))]
fn to_u5_slice(hi: u64, lo: u64) -> [u5; 27] {
    let mut dst: [u5; 27] = [0usize; 27];
    dst[0] = ((hi >> 59) & 0x1f) as u5;
    dst[1] = ((hi >> 54) & 0x1f) as u5;
    dst[2] = ((hi >> 49) & 0x1f) as u5;
    dst[3] = ((hi >> 44) & 0x1f) as u5;
    dst[4] = ((hi >> 39) & 0x1f) as u5;
    dst[5] = ((hi >> 34) & 0x1f) as u5;
    dst[6] = ((hi >> 29) & 0x1f) as u5;
    dst[7] = ((hi >> 24) & 0x1f) as u5;
    dst[8] = ((hi >> 19) & 0x1f) as u5;
    dst[9] = ((hi >> 14) & 0x1f) as u5;
    dst[10] = ((hi >> 9) & 0x1f) as u5;
    dst[11] = ((hi >> 4) & 0x1f) as u5;
    dst[12] = (((hi & 0xf) << 1) | ((lo >> 63) & 0x1)) as u5;
    //
    dst[13] = ((lo >> 58) & 0x1f) as u5;
    dst[14] = ((lo >> 53) & 0x1f) as u5;
    dst[15] = ((lo >> 48) & 0x1f) as u5;
    dst[16] = ((lo >> 43) & 0x1f) as u5;
    dst[17] = ((lo >> 38) & 0x1f) as u5;
    dst[18] = ((lo >> 33) & 0x1f) as u5;
    dst[19] = ((lo >> 28) & 0x1f) as u5;
    dst[20] = ((lo >> 23) & 0x1f) as u5;
    dst[21] = ((lo >> 18) & 0x1f) as u5;
    dst[22] = ((lo >> 13) & 0x1f) as u5;
    dst[23] = ((lo >> 8) & 0x1f) as u5;
    dst[24] = ((lo >> 3) & 0x1f) as u5;
    dst[25] = ((lo & 0x7) << 2) as u5;
    dst
}

#[cfg(not(feature = "non_binary"))]
fn to_u64_slice(slice: &[u5; 27]) -> (u64, u64) {
    let hi: u64 = ((slice[0] as u64) << 59)
        | ((slice[1] as u64) << 54)
        | ((slice[2] as u64) << 49)
        | ((slice[3] as u64) << 44)
        | ((slice[4] as u64) << 39)
        | ((slice[5] as u64) << 34)
        | ((slice[6] as u64) << 29)
        | ((slice[7] as u64) << 24)
        | ((slice[8] as u64) << 19)
        | ((slice[9] as u64) << 14)
        | ((slice[10] as u64) << 9)
        | ((slice[11] as u64) << 4)
        | (((slice[12] as u64) >> 1) & 0xf); //
    let lo: u64 = (((slice[13] as u64) << 58) | ((slice[12] as u64) & 0x1) << 63)
        | ((slice[14] as u64) << 53)
        | ((slice[15] as u64) << 48)
        | ((slice[16] as u64) << 43)
        | ((slice[17] as u64) << 38)
        | ((slice[18] as u64) << 33)
        | ((slice[19] as u64) << 28)
        | ((slice[20] as u64) << 23)
        | ((slice[21] as u64) << 18)
        | ((slice[22] as u64) << 13)
        | ((slice[23] as u64) << 8)
        | ((slice[24] as u64) << 3)
        | ((slice[25] as u64) >> 2); //
    (hi, lo)
}

#[cfg(feature = "non_binary")]
pub fn encode(euid: &EUID, checkmod: bool) -> String {
    let check = (if checkmod {
        crate::check::m7(euid)
    } else {
        0x7f
    }) as u128;

    let mut p1 = euid.0 >> 19;
    let mut p2 = ((euid.0 & 0x7ffff) << 1) | (euid.1 >> 63);
    let mut p3 = (((euid.1 as u128) & 0x7fffffffffffffff) << 7) | check;

    let mut p1_str: [char; 9] = ['0'; 9];
    let mut p2_str: [char; 4] = ['0'; 4];
    let mut p3_str: [char; 14] = ['0'; 14];

    for (_, c) in p1_str.iter_mut().enumerate() {
        let m = p1 % 32;
        *c = ENCODING_SYMBOLS[m as usize];
        p1 = (p1 - m) / 32;
    }
    for (_, c) in p2_str.iter_mut().enumerate() {
        let m = p2 % 32;
        *c = ENCODING_SYMBOLS[m as usize];
        p2 = (p2 - m) / 32;
    }
    for (_, c) in p3_str.iter_mut().enumerate() {
        let mr = p3 % 32;
        *c = ENCODING_SYMBOLS[mr as usize];
        p3 = (p3 - mr) / 32;
    }
    format!(
        "{}{}{}",
        p1_str.iter().rev().collect::<String>(),
        p2_str.iter().rev().collect::<String>(),
        p3_str.iter().rev().collect::<String>(),
    )
}

#[cfg(not(feature = "non_binary"))]
pub fn encode(euid: &EUID, checkmod: bool) -> String {
    let slice: [u5; 27] = to_u5_slice(euid.0, euid.1);
    let mut dst: String = String::with_capacity(27);
    dst.push(ENCODING_SYMBOLS[slice[0]]);
    dst.push(ENCODING_SYMBOLS[slice[1]]);
    dst.push(ENCODING_SYMBOLS[slice[2]]);
    dst.push(ENCODING_SYMBOLS[slice[3]]);
    dst.push(ENCODING_SYMBOLS[slice[4]]);
    dst.push(ENCODING_SYMBOLS[slice[5]]);
    dst.push(ENCODING_SYMBOLS[slice[6]]);
    dst.push(ENCODING_SYMBOLS[slice[7]]);
    dst.push(ENCODING_SYMBOLS[slice[8]]);
    dst.push(ENCODING_SYMBOLS[slice[9]]);
    dst.push(ENCODING_SYMBOLS[slice[10]]);
    dst.push(ENCODING_SYMBOLS[slice[11]]);
    dst.push(ENCODING_SYMBOLS[slice[12]]);
    dst.push(ENCODING_SYMBOLS[slice[13]]);
    dst.push(ENCODING_SYMBOLS[slice[14]]);
    dst.push(ENCODING_SYMBOLS[slice[15]]);
    dst.push(ENCODING_SYMBOLS[slice[16]]);
    dst.push(ENCODING_SYMBOLS[slice[17]]);
    dst.push(ENCODING_SYMBOLS[slice[18]]);
    dst.push(ENCODING_SYMBOLS[slice[19]]);
    dst.push(ENCODING_SYMBOLS[slice[20]]);
    dst.push(ENCODING_SYMBOLS[slice[21]]);
    dst.push(ENCODING_SYMBOLS[slice[22]]);
    dst.push(ENCODING_SYMBOLS[slice[23]]);
    dst.push(ENCODING_SYMBOLS[slice[24]]);
    let check: usize = if checkmod {
        crate::check::m7(euid)
    } else {
        0x7f
    };
    dst.push(ENCODING_SYMBOLS[slice[25] | (check >> 5)]);
    dst.push(ENCODING_SYMBOLS[check & 0x1f]);
    dst
}

#[cfg(feature = "non_binary")]
pub fn decode(encoded: &str) -> Result<EUID, Error> {
    if encoded.len() != 27 {
        return Err(Error::InvalidLength(encoded.len(), 27));
    }
    let mut p1: u64 = 0;
    let mut p2: u64 = 0;
    let mut p3: u128 = 0;

    let (value, rest) = encoded.split_at(9);
    for (i, c) in value.chars().rev().enumerate() {
        let code_point = c as usize;
        if code_point > DECODING_SYMBOLS.len() {
            return Err(Error::InvalidCharacter(c));
        }
        let v = DECODING_SYMBOLS[c as usize];
        if v == usize::MAX {
            return Err(Error::InvalidCharacter(c));
        }
        p1 += v as u64 * (32u64.pow(i as u32));
    }
    let (value, rest) = rest.split_at(4);
    for (i, c) in value.chars().rev().enumerate() {
        let code_point = c as usize;
        if code_point > DECODING_SYMBOLS.len() {
            return Err(Error::InvalidCharacter(c));
        }
        let v = DECODING_SYMBOLS[c as usize];
        if v == usize::MAX {
            return Err(Error::InvalidCharacter(c));
        }
        p2 += v as u64 * (32u64.pow(i as u32));
    }
    let (value, _) = rest.split_at(14);
    for (i, c) in value.chars().rev().enumerate() {
        let code_point = c as usize;
        if code_point > DECODING_SYMBOLS.len() {
            return Err(Error::InvalidCharacter(c));
        }
        let v = DECODING_SYMBOLS[c as usize];
        if v == usize::MAX {
            return Err(Error::InvalidCharacter(c));
        }
        p3 += v as u128 * (32u128.pow(i as u32));
    }
    let check = (p3 & 0x7f) as usize;

    let hi = (p1 << 19) | (p2 >> 1);
    let lo = ((p2 & 0x1) << 63) | ((p3 >> 7) & 0x7fffffffffffffff) as u64;
    let euid = EUID(hi, lo);
    if check == 0x7f {
        Ok(euid)
    } else {
        let w = crate::check::m7(&euid);
        if w == check {
            Ok(euid)
        } else {
            Err(Error::InvalidCheckmod(check, w))
        }
    }
}

#[cfg(not(feature = "non_binary"))]
pub fn decode(encoded: &str) -> Result<EUID, Error> {
    if encoded.len() != 27 {
        return Err(Error::InvalidLength(encoded.len(), 27));
    }
    let mut slice: [u5; 27] = [0usize; 27];
    for (i, c) in encoded.chars().enumerate() {
        let code_point: usize = c as usize;
        if code_point >= DECODING_SYMBOLS.len() {
            return Err(Error::InvalidCharacter(c));
        }
        slice[i] = DECODING_SYMBOLS[code_point];
        if slice[i] == usize::MAX {
            return Err(Error::InvalidCharacter(c));
        }
    }
    let r: usize = slice[25] & 0x3;
    slice[25] &= 0x1c;
    let e: (u64, u64) = to_u64_slice(&slice);
    let check: usize = (r << 5) | slice[26];
    if check == 0x7f {
        Ok(EUID(e.0, e.1))
    } else {
        let euid: EUID = EUID(e.0, e.1);
        let w = crate::check::m7(&euid);
        if check != w {
            Err(Error::InvalidCheckmod(check, w))
        } else {
            Ok(euid)
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    #[cfg(not(feature = "non_binary"))]
    #[test]
    fn convert_bits_test() {
        for _ in 0..65536 {
            let (hi, lo) = crate::random::random_u128();
            let slice: [usize; 27] = super::to_u5_slice(hi, lo);
            let (hi2, lo2) = super::to_u64_slice(&slice);
            assert_eq!(hi, hi2);
            assert_eq!(lo, lo2);
        }
    }

    #[test]
    fn encode_test() {
        for i in 0..32767 {
            let euid = crate::EUID::create_with_extension(i).unwrap();
            assert_eq!(euid, crate::EUID::from_str(&euid.encode(true)).unwrap());
        }
    }

    #[test]
    fn decode_test() {
        assert_eq!(
            "C8ZM14GR4JXG0MQXVY18S8TJNBZ",
            super::decode("C8ZM14GR4JXG0MQXVY18S8TJNBZ")
                .unwrap()
                .encode(false)
        );
        assert_eq!(
            Err(crate::Error::InvalidCheckmod(123, 56)),
            super::decode("C8X2HA87098A0W837DX13FEAWVV")
        );
        assert_eq!(
            Err(crate::Error::InvalidCharacter('U')),
            super::decode("C8EE934SR007G5Q94QKKXFRFV8U")
        );
        assert_eq!(
            Err(crate::Error::InvalidCharacter('}')),
            super::decode("C8EE934SR007G5Q94QKKXFRFV8}")
        );
        assert_eq!(
            Err(crate::Error::InvalidCharacter('@')),
            super::decode("C8EE934SR007G5Q94QKKXFRFV8@")
        );
        assert_eq!(
            Err(crate::Error::InvalidLength(26, 27)),
            super::decode("C8EE934SR007G5Q94QKKXFRFV8")
        );
        assert_eq!(
            "C8EE934SR007G5Q94QKKXFRFV8B",
            super::decode("C8EE934SR007G5Q94QKKXFRFV8B")
                .unwrap()
                .to_string()
        );
    }
}
