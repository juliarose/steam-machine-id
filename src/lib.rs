//! Used for generating Steam machine IDs. Machine IDs are most commonly supplied to Steam when 
//! logging in.
//! 
//! # Usage
//! 
//! Generating random machine IDs.
//! ```rs
//! use steam_machine_id::MachineId;
//! 
//! // Creates a random machine ID.
//! let machine_id = MachineId::random();
//! ```
//! 
//! Consuming a generated machine ID for a login request.
//! ```rs
//! use steam_machine_id::MachineId;
//! 
//! struct LoginRequest {
//!     machine_id: Vec<u8>,
//! }
//! 
//! // Creates a machine ID from the given account name.
//! let machine_id = MachineId::from_account_name("accountname");
//! let login = LoginRequest {
//!     // Converts the machine ID into a binary message object.
//!     machine_id: machine_id.into(),
//! };
//! ```

#![warn(missing_docs)]
use std::fmt::{self, Write};
use bytes::{BytesMut, BufMut};
use rand::Rng;
use sha1::{Sha1, Digest};

/// A SHA1 hash value.
type Sha1HashValue = [u8; 40];

/// The length of a SHA1 hash value.
const SHA_1_HASH_VALUE_LENGTH: usize = 40;

/// A Steam machine ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineId {
    /// The BB3 SHA1 hash value.
    pub value_bb3: Sha1HashValue,
    /// The FF2 SHA1 hash value.
    pub value_ff2: Sha1HashValue,
    /// The 3B3 SHA1 hash value.
    pub value_3b3: Sha1HashValue,
}

impl MachineId {
    /// Creates a new machine ID.
    fn new(machine_id_type: MachineIdType) -> Self {
        machine_id_type.into()
    }
    
    /// Creates a random machine ID.
    /// 
    /// # Examples
    /// ```
    /// use steam_machine_id::MachineId;
    /// 
    /// let machine_id = MachineId::random();
    /// ```
    pub fn random() -> Self {
        Self::new(MachineIdType::Random)
    }
    
    /// Creates a machine ID from the given account name.
    /// 
    /// # Examples
    /// ```
    /// use steam_machine_id::MachineId;
    /// 
    /// let machine_id = MachineId::from_account_name("accountname".into());
    /// ```
    pub fn from_account_name(account_name: &str) -> Self {
        Self::new(MachineIdType::AccountName(account_name))
    }
    
    /// Creates a message object from the machine ID.
    pub fn to_message(&self) -> Vec<u8> {
        create_machine_id_from_values(
            &self.value_bb3,
            &self.value_ff2,
            &self.value_3b3,
        )
    }
}

impl From<MachineId> for Vec<u8> {
    fn from(machine_id: MachineId) -> Self {
        machine_id.to_message()
    }
}

impl From<&MachineId> for Vec<u8> {
    fn from(machine_id: &MachineId) -> Self {
        machine_id.to_message()
    }
}

impl fmt::Display for MachineId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BB3.{}:FF2.{}:3B3.{}", 
            bytes_to_hex_string(&self.value_bb3),
            bytes_to_hex_string(&self.value_ff2),
            bytes_to_hex_string(&self.value_3b3),
        )
    }
}

impl From<MachineIdType<'_>> for MachineId {
    fn from(machine_id_type: MachineIdType<'_>) -> Self {
        match machine_id_type {
            MachineIdType::Random => {
                MachineId {
                    value_bb3: get_random_hash_value(),
                    value_ff2: get_random_hash_value(),
                    value_3b3: get_random_hash_value(),
                }
            },
            MachineIdType::AccountName(account_name) => {
                MachineId {
                    value_bb3: get_account_name_hash_value("BB3", account_name),
                    value_ff2: get_account_name_hash_value("FF2", account_name),
                    value_3b3: get_account_name_hash_value("3B3", account_name),
                }
            },
        }
    }
}

/// Options for creating a Steam machine ID.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MachineIdType<'a> {
    /// A random machine ID.
    Random,
    /// A machine ID created from the given account name.
    AccountName(&'a str),
}

/// Creates a machine id from the given values.
/// 
/// Each value must be a 40-byte SHA1 hash value (not null-terminated).
fn create_machine_id_from_values(
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
    buffer.put_i8(0); // 1 byte, total 61
    
    buffer.put_i8(1); // 1 byte, total 62
    buffer.put(get_c_string_bytes("FF2").as_slice()); // 4 bytes, total 66
    buffer.put(val_ff2); // 40 bytes, total 106
    // null terminator
    buffer.put_i8(0); // 1 byte, total 107
    
    buffer.put_i8(1); // 1 byte, total 108
    buffer.put(get_c_string_bytes("3B3").as_slice()); // 4 bytes, total 112
    buffer.put(val_3b3); // 40 bytes, total 152
    // null terminator
    buffer.put_i8(0); // 1 byte, total 153
    
    buffer.put_i8(8); // 1 byte, total 154
    buffer.put_i8(8); // 1 byte, total 155
    buffer.into()
}

/// Gets a random SHA1 hash value.
fn get_random_hash_value() -> Sha1HashValue {
    create_sha1_value(&create_random_str())
}

/// Gets a SHA1 hash value for the given `key` and `account_name`.
fn get_account_name_hash_value(key: &str, account_name: &str) -> Sha1HashValue {
    create_sha1_value(&format!("SteamUser Hash {key} {account_name}"))
}

/// Converts a byte array to a hex string.
fn bytes_to_hex_string(input: &[u8]) -> String {
    input
        .into_iter()
        .fold(String::with_capacity(2 * input.len()), |mut s, byte| {
            // Probably would never panic?
            write!(s, "{:02X}", byte).unwrap();
            s
        })
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

/// Gets a null-terminated (C string) byte array from the given string.
fn get_c_string_bytes(input: &str) -> Vec<u8> {
    let mut bytes = input.as_bytes().to_vec();
    
    bytes.push(0);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn tests_bytes_to_hex_string() {
        let bytes = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let hex_string = bytes_to_hex_string(&bytes);
        
        assert_eq!(hex_string, "0001020304050607");
    }
    
    #[test]
    fn tests_get_c_string_bytes() {
        let bytes = get_c_string_bytes("test");
        
        assert_eq!(bytes.len(), 5);
        assert_eq!([116, 101, 115, 116, 0], bytes.as_slice());
    }
    
    #[test]
    fn test_create_sha1() {
        let sha1 = create_sha1(b"test");
        
        assert_eq!(sha1.len(), 20);
    }
    
    #[test]
    fn test_random_machine_id() {
        let machine_id = MachineId::random().to_message();
        
        assert_eq!(machine_id.len(), 155);
        assert_eq!(machine_id[0], 0);
        assert_eq!(&machine_id[1..15], get_c_string_bytes("MessageObject").as_slice());
        assert_eq!(machine_id[15], 1);
        assert_eq!(&machine_id[16..20], get_c_string_bytes("BB3").as_slice());
        assert_eq!(machine_id[61], 1);
        assert_eq!(&machine_id[62..66], get_c_string_bytes("FF2").as_slice());
        assert_eq!(machine_id[107], 1);
        assert_eq!(&machine_id[108..112], get_c_string_bytes("3B3").as_slice());
        assert_eq!(machine_id[153], 8);
        assert_eq!(machine_id[154], 8);
    }
    
    #[test]
    fn test_create_machine_id_from_account_name() {
        let machine_id = MachineId::from_account_name("accountname").to_message();
        
        assert_eq!(machine_id.len(), 155);
        assert_eq!(machine_id[0], 0);
        assert_eq!(&machine_id[1..15], get_c_string_bytes("MessageObject").as_slice());
        assert_eq!(machine_id[15], 1);
        assert_eq!(&machine_id[16..20], get_c_string_bytes("BB3").as_slice());
        assert_eq!(machine_id[61], 1);
        assert_eq!(&machine_id[62..66], get_c_string_bytes("FF2").as_slice());
        assert_eq!(machine_id[107], 1);
        assert_eq!(&machine_id[108..112], get_c_string_bytes("3B3").as_slice());
        assert_eq!(machine_id[153], 8);
        assert_eq!(machine_id[154], 8);
    }
    
    #[test]
    fn tests_machine_id() {
        let machine_id = MachineId::from_account_name("accountname");
        
        assert_eq!(String::from_utf8_lossy(&machine_id.value_bb3), "6BB2445F8825BFED65E64392F0A4D549FFF7D3E1");
        assert_eq!(String::from_utf8_lossy(&machine_id.value_ff2), "57AD645E54976AFF3B3662E9CB335D0A24AC7D08");
        assert_eq!(String::from_utf8_lossy(&machine_id.value_3b3), "C1884025D23FB1A0DDBF125B5D9B8C0812F83390");
    }
}
