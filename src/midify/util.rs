pub fn to_bytes(num: usize) -> Vec<u8> {
    let num: u32 = num.try_into().unwrap();

    let upper = num >> 24;
    let upper_mid = 0xff & (num >> 16);
    let lower_mid = 0xff & (num >> 8);
    let lower = 0xff & num;

    let upper: u8 = upper.try_into().unwrap();
    let upper_mid: u8 = upper_mid.try_into().unwrap();
    let lower_mid: u8 = lower_mid.try_into().unwrap();
    let lower: u8 = lower.try_into().unwrap();
    vec![upper, upper_mid, lower_mid, lower]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_bytes() {
        assert_eq!(to_bytes(0xaabbccdd), vec![0xaa, 0xbb, 0xcc, 0xdd])
    }
}
