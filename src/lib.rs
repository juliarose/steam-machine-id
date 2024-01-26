#![warn(missing_docs)]

//! Used for generating Steam machine IDs. Machine IDs are most commonly supplied to Steam when 
//! logging in.
//! 
//! # Usage
//! 
//! Generating random machine IDs.
//! ```rs
//! use steam_machine_id::MachineID;
//! 
//! // Creates a random machine ID.
//! let machine_id = MachineID::random();
//! ```
//! 
//! Consuming a generated machine ID for a login request.
//! ```rs
//! use steam_machine_id::MachineID;
//! 
//! struct LoginRequest {
//!     machine_id: Vec<u8>,
//! }
//! 
//! // Creates a machine ID from the given account name.
//! let machine_id = MachineID::from_account_name("accountname");
//! let login = LoginRequest {
//!     // Converts the machine ID into a binary message object.
//!     machine_id: machine_id.into(),
//! };
//! ```

mod helpers;

use std::fmt;
use helpers::Sha1HashValue;

/// A Steam machine ID.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MachineID {
    /// The BB3 SHA1 hash value. This value is not hexadecimal encoded.
    pub value_bb3: Sha1HashValue,
    /// The FF2 SHA1 hash value. This value is not hexadecimal encoded.
    pub value_ff2: Sha1HashValue,
    /// The 3B3 SHA1 hash value. This value is not hexadecimal encoded.
    pub value_3b3: Sha1HashValue,
}

impl MachineID {
    /// Creates a new machine ID.
    fn new(machine_id_type: MachineIDType) -> Self {
        machine_id_type.into()
    }
    
    /// Creates a random machine ID.
    /// 
    /// # Examples
    /// ```
    /// use steam_machine_id::MachineID;
    /// 
    /// let machine_id = MachineID::random();
    /// ```
    pub fn random() -> Self {
        Self::new(MachineIDType::Random)
    }
    
    /// Creates a machine ID from the given account name.
    /// 
    /// # Examples
    /// ```
    /// use steam_machine_id::MachineID;
    /// 
    /// let machine_id = MachineID::from_account_name("accountname".into());
    /// ```
    pub fn from_account_name(account_name: &str) -> Self {
        Self::new(MachineIDType::AccountName(account_name))
    }
    
    /// Creates a machine ID using a custom format for specific use-cases. These could be anything 
    /// you want but should generally follow the format below.
    /// 
    /// # Examples
    /// ```
    /// use steam_machine_id::MachineID;
    /// 
    /// let accountname = "accountname";
    /// let machine_id = MachineID::custom_format(
    ///     &format!("SteamUser Hash BB3 {accountname}"),
    ///     &format!("SteamUser Hash FF2 {accountname}"),
    ///     &format!("SteamUser Hash 3B3 {accountname}"),
    /// );
    /// ```
    pub fn custom_format(
        value_bb3: &str,
        value_ff2: &str,
        value_3b3: &str,
    ) -> Self {
        Self::new(MachineIDType::CustomFormat(
            value_bb3,
            value_ff2,
            value_3b3,
        ))
    }
    
    /// Creates a message object from the machine ID.
    pub fn to_message(&self) -> Vec<u8> {
        helpers::create_machine_id_from_values(
            &self.value_bb3,
            &self.value_ff2,
            &self.value_3b3,
        )
    }
}

impl From<MachineID> for Vec<u8> {
    fn from(machine_id: MachineID) -> Self {
        machine_id.to_message()
    }
}

impl From<&MachineID> for Vec<u8> {
    fn from(machine_id: &MachineID) -> Self {
        machine_id.to_message()
    }
}

impl fmt::Display for MachineID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BB3.{}:FF2.{}:3B3.{}", 
            helpers::bytes_to_hex_string(&self.value_bb3),
            helpers::bytes_to_hex_string(&self.value_ff2),
            helpers::bytes_to_hex_string(&self.value_3b3),
        )
    }
}

impl From<MachineIDType<'_>> for MachineID {
    fn from(machine_id_type: MachineIDType<'_>) -> Self {
        match machine_id_type {
            MachineIDType::Random => {
                MachineID {
                    value_bb3: helpers::get_random_hash_value(),
                    value_ff2: helpers::get_random_hash_value(),
                    value_3b3: helpers::get_random_hash_value(),
                }
            },
            MachineIDType::AccountName(account_name) => {
                MachineID {
                    value_bb3: helpers::get_account_name_hash_value("BB3", account_name),
                    value_ff2: helpers::get_account_name_hash_value("FF2", account_name),
                    value_3b3: helpers::get_account_name_hash_value("3B3", account_name),
                }
            },
            MachineIDType::CustomFormat(value_bb3, value_ff2, value_3b3) => {
                MachineID {
                    value_bb3: helpers::get_custom_hash_value(value_bb3),
                    value_ff2: helpers::get_custom_hash_value(value_ff2),
                    value_3b3: helpers::get_custom_hash_value(value_3b3),
                }
            },
        }
    }
}

/// Options for creating a Steam machine ID.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MachineIDType<'a> {
    /// A random machine ID.
    Random,
    /// A machine ID created from the given account name.
    AccountName(&'a str),
    /// A machine ID created using a custom format.
    CustomFormat(&'a str, &'a str, &'a str),
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::{get_c_string, bytes_to_hex_string};
    
    #[test]
    fn test_random_machine_id() {
        let machine_id = MachineID::random().to_message();
        
        assert_eq!(machine_id.len(), 155);
        assert_eq!(machine_id[0], 0);
        assert_eq!(&machine_id[1..15], get_c_string("MessageObject").as_slice());
        assert_eq!(machine_id[15], 1);
        assert_eq!(&machine_id[16..20], get_c_string("BB3").as_slice());
        assert_eq!(machine_id[61], 1);
        assert_eq!(&machine_id[62..66], get_c_string("FF2").as_slice());
        assert_eq!(machine_id[107], 1);
        assert_eq!(&machine_id[108..112], get_c_string("3B3").as_slice());
        assert_eq!(machine_id[153], 8);
        assert_eq!(machine_id[154], 8);
    }
    
    #[test]
    fn test_create_machine_id_from_account_name() {
        let machine_id = MachineID::from_account_name("accountname").to_message();
        
        assert_eq!(machine_id.len(), 155);
        assert_eq!(machine_id[0], 0);
        assert_eq!(&machine_id[1..15], get_c_string("MessageObject").as_slice());
        assert_eq!(machine_id[15], 1);
        assert_eq!(&machine_id[16..20], get_c_string("BB3").as_slice());
        assert_eq!(machine_id[61], 1);
        assert_eq!(&machine_id[62..66], get_c_string("FF2").as_slice());
        assert_eq!(machine_id[107], 1);
        assert_eq!(&machine_id[108..112], get_c_string("3B3").as_slice());
        assert_eq!(machine_id[153], 8);
        assert_eq!(machine_id[154], 8);
    }
    
    #[test]
    fn tests_machine_id() {
        let machine_id = MachineID::from_account_name("accountname");
        
        assert_eq!(bytes_to_hex_string(&machine_id.value_bb3), "6BB2445F8825BFED65E64392F0A4D549FFF7D3E1");
        assert_eq!(bytes_to_hex_string(&machine_id.value_ff2), "57AD645E54976AFF3B3662E9CB335D0A24AC7D08");
        assert_eq!(bytes_to_hex_string(&machine_id.value_3b3), "C1884025D23FB1A0DDBF125B5D9B8C0812F83390");
    }
}
