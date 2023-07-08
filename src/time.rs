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
