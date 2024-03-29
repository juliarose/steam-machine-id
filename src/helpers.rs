use std::fmt::Write;
use std::ffi::CString;
use bytes::{BytesMut, BufMut};
use rand::Rng;
use sha1_smol::Sha1;

/// A SHA1 hash value.
pub type Sha1HashValue = [u8; 20];

/// Creates a machine id from the given SHA-1 hash values.
pub fn create_machine_id_from_values(
    value_bb3: &Sha1HashValue,
    value_ff2: &Sha1HashValue,
    value_3b3: &Sha1HashValue,
) -> Vec<u8> {
    let mut buffer = BytesMut::with_capacity(155);
    let value_bb3 = get_c_string(&bytes_to_hex_string(value_bb3));
    let value_ff2 = get_c_string(&bytes_to_hex_string(value_ff2));
    let value_3b3 = get_c_string(&bytes_to_hex_string(value_3b3));
    
    buffer.put_i8(0); // 1 byte, total 1
    buffer.put(get_c_string("MessageObject").as_slice()); // 14 bytes, total 15
    
    buffer.put_i8(1); // 1 byte, total 16
    buffer.put(get_c_string("BB3").as_slice()); // 4 bytes, total 20
    buffer.put(value_bb3.as_slice()); // 41 bytes, total 61
    
    buffer.put_i8(1); // 1 byte, total 62
    buffer.put(get_c_string("FF2").as_slice()); // 4 bytes, total 66
    buffer.put(value_ff2.as_slice()); // 41 bytes, total 107
    
    buffer.put_i8(1); // 1 byte, total 108
    buffer.put(get_c_string("3B3").as_slice()); // 4 bytes, total 112
    buffer.put(value_3b3.as_slice()); // 41 bytes, total 153
    
    buffer.put_i8(8); // 1 byte, total 154
    buffer.put_i8(8); // 1 byte, total 155
    buffer.into()
}

/// Converts a byte slice to a hex string.
pub fn bytes_to_hex_string(input: &[u8]) -> String {
    input
        .iter()
        .fold(String::with_capacity(2 * input.len()), |mut s, byte| {
            // Probably would never panic?
            write!(s, "{:02X}", byte).unwrap();
            s
        })
}

/// Gets a random SHA1 hash value.
pub fn get_random_hash_value() -> Sha1HashValue {
    create_sha1(create_random_str().as_bytes())
}

/// Gets a SHA1 hash value for the given `key` and `account_name`.
pub fn get_account_name_hash_value(key: &str, account_name: &str) -> Sha1HashValue {
    create_sha1(format!("SteamUser Hash {key} {account_name}").as_bytes())
}

/// Gets a SHA1 hash value for the given `key` and `account_name`.
pub fn get_custom_hash_value(value: &str) -> Sha1HashValue {
    create_sha1(value.as_bytes())
}

/// Gets a null-terminated (C string) byte vec from the given string.
pub fn get_c_string(input: &str) -> Vec<u8> {
    // As long as no null bytes ("\0") are in the string, this will never panic.
    CString::new(input).unwrap().into_bytes_with_nul()
}

/// Creates a SHA1 byte slice from the given input.
fn create_sha1(input: &[u8]) -> Sha1HashValue {
    Sha1::from(input).digest().bytes()
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
    
    #[test]
    fn tests_get_random_hash_value() {
        let hash_value = get_random_hash_value();
        
        assert_eq!(hash_value.len(), 20);
    }
    
    #[test]
    fn tests_get_custom_hash_value() {
        let hash_value = get_custom_hash_value("test");
        
        assert_eq!(hash_value.len(), 20);
    }
    
    #[test]
    fn tests_get_account_name_hash_value() {
        let hash_value = get_account_name_hash_value("test", "test");
        
        assert_eq!(hash_value.len(), 20);
    }
    
    #[test]
    #[should_panic]
    fn test_get_random_hash_value() {
        get_c_string("\0");
    }
    
    #[test]
    fn tests_bytes_to_hex_string() {
        let bytes = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let hex_string = bytes_to_hex_string(&bytes);
        
        assert_eq!(hex_string, "0001020304050607");
    }
    
    #[test]
    fn tests_get_c_string_bytes() {
        let bytes = get_c_string("test");
        
        assert_eq!(bytes.as_slice().len(), 5);
        assert_eq!([116, 101, 115, 116, 0], bytes.as_slice());
    }
}