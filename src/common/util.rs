pub fn bytes_to_u32(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    u32::from(b0) << 24 | u32::from(b1) << 16 | u32::from(b2) << 8 | u32::from(b3)
}
