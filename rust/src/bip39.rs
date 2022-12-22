use bip39::{Language, Mnemonic};

pub fn validate(mnemonic: &str) -> bool {
    Mnemonic::validate(&mnemonic, Language::English).is_ok()
}
