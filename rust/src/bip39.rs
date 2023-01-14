use std::str::FromStr;

use bip39::{Language, Mnemonic};

pub fn validate(mnemonic: &str) -> bool {
    match Mnemonic::from_str(&mnemonic) {
        Ok(_) => true,
        Err(_) => false,
    }
}
