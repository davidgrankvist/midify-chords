const MICROSECONDS_PER_MINUTE: u32 = 60_000_000;

pub fn to_bytes(num: u32) -> Vec<u8> {
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

fn bpm_to_tempo(bpm: u32) -> u32 {
    ((MICROSECONDS_PER_MINUTE as f64) / bpm as f64).round() as u32
}

pub fn bpm_to_tempo_bytes(bpm: u32) -> Vec<u8> {
    let bytes = to_bytes(bpm_to_tempo(bpm));
    bytes[1..].to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_bytes() {
        assert_eq!(to_bytes(0xaabbccdd), vec![0xaa, 0xbb, 0xcc, 0xdd])
    }

    #[test]
    fn test_bpm_to_tempo() {
        assert_eq!(bpm_to_tempo(120), 500000);
        assert_eq!(bpm_to_tempo(60), 1000000);
        assert_eq!(bpm_to_tempo(240), 250000);
    }

    #[test]
    fn test_bpm_to_tempo_bytes() {
        assert_eq!(bpm_to_tempo_bytes(120), vec![0x07, 0xa1, 0x20]);
        assert_eq!(bpm_to_tempo_bytes(60), vec![0x0f, 0x42, 0x40]);
        assert_eq!(bpm_to_tempo_bytes(240), vec![0x03, 0xd0, 0x90]);
    }
}
