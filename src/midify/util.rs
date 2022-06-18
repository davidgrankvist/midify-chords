const QUARTER_DELTA: u8 = 0x19;

pub fn to_midi_delta(index: usize) -> Vec<u8> {
    let index: u16 = index.try_into().unwrap();
    let delta: u16 = QUARTER_DELTA.try_into().unwrap();
    let delta = index * delta;

    let upper = 0x80 | ( delta >> 8 );
    let lower = 0x00ff & delta;
    vec![upper.try_into().unwrap(), lower.try_into().unwrap()]
}

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
    fn test_to_midi_delta() {
        assert_eq!(to_midi_delta(0), vec![0x80, 0x00]);
        assert_eq!(to_midi_delta(1), vec![0x80, QUARTER_DELTA]);
        assert_eq!(to_midi_delta(100), vec![0x89, 0xc4]);
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(to_bytes(0xaabbccdd), vec![0xaa, 0xbb, 0xcc, 0xdd])
    }
}
