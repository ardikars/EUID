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

fn normalize_timestamp(now: u64, epoch: u64) -> u64 {
    if epoch < now {
        now - epoch
    } else {
        now
    }
}

pub fn get_timestamp_from_epoch(epoch: u64) -> u64 {
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

#[cfg(test)]
mod tests {
    use crate::time::normalize_timestamp;

    #[test]
    fn normalize_timestamp_test() {
        assert_eq!(1, normalize_timestamp(1, 2));
        assert_eq!(1, normalize_timestamp(2, 1));
    }
}
