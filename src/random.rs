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

#[allow(unused_imports)]
use getrandom::getrandom;

pub fn random_u32() -> u32 {
    let mut r: [u8; 4] = [0u8; 4];
    match getrandom::getrandom(&mut r) {
        Ok(_) => u32::from_be_bytes(r),
        Err(_) => 0,
    }
}

pub fn random_u64() -> u64 {
    let mut r: [u8; 8] = [0u8; 8];
    match getrandom::getrandom(&mut r) {
        Ok(_) => u64::from_be_bytes(r),
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {

    use crate::random::{random_u32, random_u64};

    #[test]
    fn random_test() {
        assert!((random_u32() as u64) < random_u64());
    }
}
