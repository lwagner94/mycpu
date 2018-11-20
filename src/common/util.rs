pub fn bytes_to_u32(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    u32::from(b0) << 24 | u32::from(b1) << 16 | u32::from(b2) << 8 | u32::from(b3)
}

pub fn u32_to_bytes(value: u32) -> [u8; 4] {
    [
        ((value & 0xFF_00_00_00) >> 24) as u8,
        ((value & 0x00_FF_00_00) >> 16) as u8,
        ((value & 0x00_00_FF_00) >> 8) as u8,
        (value & 0x00_00_00_FF) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_u32() {
        let bytes = [0xAA, 0xBB, 0xCC, 0xDD];
        let result = bytes_to_u32(bytes[0], bytes[1], bytes[2], bytes[3]);
        assert_eq!(result, 0xAABBCCDD);
    }

    #[test]
    fn test_u32_to_bytes() {
        let bytes = [0xAA, 0xBB, 0xCC, 0xDD];
        let result = u32_to_bytes(0xAABBCCDD);
        assert_eq!(result, bytes);
    }
}
