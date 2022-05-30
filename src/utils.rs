pub fn concat_bytes(bytes: &[u8]) -> u64 {
    let mut result = bytes[0] as u64;
    for i in 1..bytes.len() {
        result = (result << 8) | (bytes[i] as u64)
    }
    return result;
}
