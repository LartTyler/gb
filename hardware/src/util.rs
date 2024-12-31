/// "Splits" a `u16` into a byte pair. The first element in the returned array is the low byte,
/// and the second is the high byte.
pub fn word_to_bytes(value: u16) -> [u8; 2] {
    value.to_le_bytes()
}

/// "Combines" two bytes into a 16-bit (word) value.
pub fn bytes_to_word(high: u8, low: u8) -> u16 {
    u16::from_le_bytes([low, high])
}
