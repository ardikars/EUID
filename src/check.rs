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

use crate::EUID;

#[cfg(not(feature = "std"))]
#[allow(dead_code)]
fn shift_right7(v: (u64, u64)) -> (u64, u64) {
    let mask: u64 = 0x7f;
    let a1: u64 = v.0 & mask;
    let a: u64 = v.0 >> 7;
    let b: u64 = (v.1 >> 7) | (a1 << 57);
    (a, b)
}

#[cfg(not(feature = "std"))]
#[allow(dead_code)]
fn add_u128(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    let mut a1 = a.0;
    let mut a2 = a.1;
    let mut b1 = b.0;
    let mut b2 = b.1;

    let mut sum1: u64 = 0;
    let mut sum2: u64 = 0;
    let mut carry1: u64 = 0;
    let mut carry2: u64 = 1;
    while (carry1 != 0) || (carry2 != 0) {
        sum1 = a1 ^ b1;
        sum2 = a2 ^ b2;
        let a2b2: u64 = a2 & b2;
        carry2 = a2b2 << 1;
        carry1 = ((a1 & b1) << 1) | (a2b2 >> 63);
        a1 = sum1;
        a2 = sum2;
        b1 = carry1;
        b2 = carry2;
    }
    (sum1, sum2)
}

#[cfg(not(feature = "std"))]
#[allow(dead_code)]
fn sub_u128(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    let (r1, r2) = add_u128((!b.0, !b.1), (0, 1));
    add_u128((a.0, a.1), (r1, r2))
}

#[cfg(not(feature = "std"))]
#[allow(dead_code)]
fn is_gt_p(v: (u64, u64), p: u64) -> bool {
    if v.0 != 0 {
        true
    } else {
        v.1 > p
    }
}

#[cfg(not(feature = "std"))]
#[allow(dead_code)]
pub fn m7(euid: &EUID) -> usize {
    let p: u64 = 0x7f;
    let mut i = add_u128((0, euid.1 & p), shift_right7((euid.0, euid.1)));
    while is_gt_p(i, p) {
        i = add_u128((0, i.1 & p), shift_right7((i.0, i.1)));
    }
    if (i.0 == 0) && (i.1 == p) {
        0
    } else {
        i.1 as usize
    }
}

#[cfg(feature = "std")]
#[allow(dead_code)]
pub fn m7(euid: &EUID) -> usize {
    let p: u128 = 0x7f;
    let n = ((euid.0 as u128) << 64) | (euid.1 as u128);
    let mut i = (n & p) + (n >> 7);
    while i > p {
        i = (i & p) + (i >> 7);
    }
    if i == p {
        0
    } else {
        i as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{random::*, EUID};

    fn to_u128(hi: u64, lo: u64) -> u128 {
        ((hi as u128) << 64) | (lo as u128)
    }

    #[cfg(not(feature = "std"))]
    #[test]
    fn add_u128_test() {
        for _ in 0..65535 {
            let hi_a: u64 = random_u32() as u64;
            let lo_a: u64 = random_u64();
            let hi_b: u64 = random_u64();
            let lo_b: u64 = random_u64();

            let a: u128 = to_u128(hi_a, lo_a);
            let b: u128 = to_u128(hi_b, lo_b);
            let ab = a + b;
            let (hi, lo) = add_u128((hi_a, lo_a), (hi_b, lo_b));
            let c = to_u128(hi, lo);
            assert_eq!(ab, c);
        }
    }

    #[cfg(not(feature = "std"))]
    #[test]
    fn sub_u128_test() {
        for _ in 0..65535 {
            let hi_a: u64 = random_u64();
            let lo_a: u64 = random_u64();
            let hi_b: u64 = random_u32() as u64;
            let lo_b: u64 = random_u64();

            let a: u128 = to_u128(hi_a, lo_a);
            let b: u128 = to_u128(hi_b, lo_b);
            let ab = a - b;
            let (hi, lo) = sub_u128((hi_a, lo_a), (hi_b, lo_b));
            let c = to_u128(hi, lo);
            assert_eq!(ab, c);
        }
    }

    #[cfg(not(feature = "std"))]
    #[test]
    fn shift_right_test() {
        for _ in 0..65535 {
            let hi: u64 = random_u64();
            let lo: u64 = random_u64();
            let n: u128 = to_u128(hi, lo);
            let (l, r) = shift_right7((hi, lo));
            assert_eq!(n >> 7, to_u128(l, r));
        }
    }

    #[cfg(not(feature = "std"))]
    #[test]
    fn divmod_test() {
        for _ in 0..65535 {
            let hi: u64 = random_u64() as u64;
            let lo: u64 = random_u64() as u64;
            let n = (to_u128(hi, lo) % 127) as usize;
            let code = m7(&EUID(hi, lo));
            assert_eq!(n, code);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn divmod_test() {
        for _ in 0..65535 {
            let hi: u64 = random_u64() as u64;
            let lo: u64 = random_u64() as u64;
            let n = (to_u128(hi, lo) % 127) as usize;
            let code = m7(&EUID(hi, lo));
            assert_eq!(n, code);
        }
    }
}
