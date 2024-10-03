// ENCRYPTION LOGIC

pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .zip(key.iter().cycle())
        .map(|(&d, &k)| d ^ k)
        .collect()
}

pub fn decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    encrypt(data, key)
}
