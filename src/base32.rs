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

use std::iter::FromIterator;

use crate::{check, Error, EUID};

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

pub fn encode(euid: &EUID, checkmod: bool) -> String {
    let slice: [u5; 27] = to_u5_slice(euid.0, euid.1);
    let mut dst: [char; 27] = ['0'; 27];
    dst[0] = ENCODING_SYMBOLS[slice[0]];
    dst[1] = ENCODING_SYMBOLS[slice[1]];
    dst[2] = ENCODING_SYMBOLS[slice[2]];
    dst[3] = ENCODING_SYMBOLS[slice[3]];
    dst[4] = ENCODING_SYMBOLS[slice[4]];
    dst[5] = ENCODING_SYMBOLS[slice[5]];
    dst[6] = ENCODING_SYMBOLS[slice[6]];
    dst[7] = ENCODING_SYMBOLS[slice[7]];
    dst[8] = ENCODING_SYMBOLS[slice[8]];
    dst[9] = ENCODING_SYMBOLS[slice[9]];
    dst[10] = ENCODING_SYMBOLS[slice[10]];
    dst[11] = ENCODING_SYMBOLS[slice[11]];
    dst[12] = ENCODING_SYMBOLS[slice[12]];
    dst[13] = ENCODING_SYMBOLS[slice[13]];
    dst[14] = ENCODING_SYMBOLS[slice[14]];
    dst[15] = ENCODING_SYMBOLS[slice[15]];
    dst[16] = ENCODING_SYMBOLS[slice[16]];
    dst[17] = ENCODING_SYMBOLS[slice[17]];
    dst[18] = ENCODING_SYMBOLS[slice[18]];
    dst[19] = ENCODING_SYMBOLS[slice[19]];
    dst[20] = ENCODING_SYMBOLS[slice[20]];
    dst[21] = ENCODING_SYMBOLS[slice[21]];
    dst[22] = ENCODING_SYMBOLS[slice[22]];
    dst[23] = ENCODING_SYMBOLS[slice[23]];
    dst[24] = ENCODING_SYMBOLS[slice[24]];
    let check: usize = if checkmod { check::m7(euid) } else { 0x7f };
    dst[25] = ENCODING_SYMBOLS[slice[25] | (check >> 5)];
    dst[26] = ENCODING_SYMBOLS[check & 0x1f];
    String::from_iter(dst)
}

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
        let w = check::m7(&euid);
        if check != w {
            Err(Error::InvalidCheckmod(check, w))
        } else {
            Ok(euid)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        base32::{to_u5_slice, to_u64_slice},
        random, Error,
    };

    use super::decode;

    #[test]
    fn convert_bits_test() {
        for _ in 0..65536 {
            let hi: u64 = random::random_u64();
            let lo: u64 = random::random_u64();
            let slice: [usize; 27] = to_u5_slice(hi, lo);
            let (hi2, lo2) = to_u64_slice(&slice);
            assert_eq!(hi, hi2);
            assert_eq!(lo, lo2);
        }
    }

    #[test]
    fn decode_test() {
        assert_eq!(
            "C8ZM14GR4JXG0MQXVY18S8TJNBZ",
            decode("C8ZM14GR4JXG0MQXVY18S8TJNBZ").unwrap().encode(false)
        );
        assert_eq!(
            Err(Error::InvalidCheckmod(123, 56)),
            decode("C8X2HA87098A0W837DX13FEAWVV")
        );
        assert_eq!(
            Err(Error::InvalidCharacter('U')),
            decode("C8EE934SR007G5Q94QKKXFRFV8U")
        );
        assert_eq!(
            Err(Error::InvalidCharacter('}')),
            decode("C8EE934SR007G5Q94QKKXFRFV8}")
        );
        assert_eq!(
            Err(Error::InvalidCharacter('@')),
            decode("C8EE934SR007G5Q94QKKXFRFV8@")
        );
        assert_eq!(
            Err(Error::InvalidLength(26, 27)),
            decode("C8EE934SR007G5Q94QKKXFRFV8")
        );
        assert_eq!(
            "C8EE934SR007G5Q94QKKXFRFV8B",
            decode("C8EE934SR007G5Q94QKKXFRFV8B").unwrap().to_string()
        );
    }
}
