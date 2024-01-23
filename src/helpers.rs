use std::fmt::Write;
use bytes::{BytesMut, BufMut};
use rand::Rng;
use sha1::{Sha1, Digest};

/// A SHA1 hash value.
pub type Sha1HashValue = [u8; 40];

/// The length of a SHA1 hash value.
const SHA_1_HASH_VALUE_LENGTH: usize = 40;

/// Creates a machine id from the given values.
/// 
/// Each value must be a 40-byte SHA1 hash value (not null-terminated).
pub fn create_machine_id_from_values(
    val_bb3: &[u8],
    val_ff2: &[u8],
    val_3b3: &[u8],
) -> Vec<u8> {
    let mut buffer = BytesMut::with_capacity(155);
    
    buffer.put_i8(0); // 1 byte, total 1
    buffer.put(get_c_string_bytes("MessageObject").as_slice()); // 14 bytes, total 15
    
    buffer.put_i8(1); // 1 byte, total 16
    buffer.put(get_c_string_bytes("BB3").as_slice()); // 4 bytes, total 20
    buffer.put(val_bb3); // 40 bytes, total 60
    // null terminator
    buffer.put([0].as_slice()); // 1 byte, total 61
    
    buffer.put_i8(1); // 1 byte, total 62
    buffer.put(get_c_string_bytes("FF2").as_slice()); // 4 bytes, total 66
    buffer.put(val_ff2); // 40 bytes, total 106
    // null terminator
    buffer.put([0].as_slice()); // 1 byte, total 107
    
    buffer.put_i8(1); // 1 byte, total 108
    buffer.put(get_c_string_bytes("3B3").as_slice()); // 4 bytes, total 112
    buffer.put(val_3b3); // 40 bytes, total 152
    // null terminator
    buffer.put([0].as_slice()); // 1 byte, total 153
    
    buffer.put_i8(8); // 1 byte, total 154
    buffer.put_i8(8); // 1 byte, total 155
    buffer.into()
}

/// Converts a byte array to a hex string.
pub fn bytes_to_hex_string(input: &[u8]) -> String {
    input
        .into_iter()
        .fold(String::with_capacity(2 * input.len()), |mut s, byte| {
            // Probably would never panic?
            write!(s, "{:02X}", byte).unwrap();
            s
        })
}

/// Gets a random SHA1 hash value.
pub fn get_random_hash_value() -> Sha1HashValue {
    create_sha1_value(&create_random_str())
}

/// Gets a SHA1 hash value for the given `key` and `account_name`.
pub fn get_account_name_hash_value(key: &str, account_name: &str) -> Sha1HashValue {
    create_sha1_value(&format!("SteamUser Hash {key} {account_name}"))
}

/// Gets a null-terminated (C string) byte array from the given string.
pub fn get_c_string_bytes(input: &str) -> Vec<u8> {
    let mut bytes = input.as_bytes().to_vec();
    
    bytes.push(0);
    bytes
}

/// Creates a SHA1 byte array from the given input.
fn create_sha1(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    
    hasher.update(input);
    hasher.finalize().to_vec()
}

/// Creates a SHA1 string from the given input.
fn create_sha1_string(input: &str) -> String {
    let sha_bytes = create_sha1(input.as_bytes());
    
    bytes_to_hex_string(&sha_bytes)
}

fn create_sha1_value(input: &str) -> Sha1HashValue {
    let sha1_string = create_sha1_string(input);
    let mut bytes = [0u8; SHA_1_HASH_VALUE_LENGTH];
    bytes.copy_from_slice(sha1_string.as_bytes());
    bytes
}

/// Creates a random string.
fn create_random_str() -> String {
    rand::thread_rng().gen::<f32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_sha1() {
        let sha1 = create_sha1(b"test");
        
        assert_eq!(sha1.len(), 20);
    }
}