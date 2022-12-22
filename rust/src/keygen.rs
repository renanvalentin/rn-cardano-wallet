use bip39::{Language, Mnemonic};
use cardano_serialization_lib::{
    address::{BaseAddress, StakeCredential},
    crypto::{Bip32PrivateKey, Bip32PublicKey},
};

fn harden(index: u32) -> u32 {
    index | 0x80_00_00_00
}

pub struct Bech32PrivateKey {
    value: String,
}

impl Bech32PrivateKey {
    pub fn new(value: String) -> Bech32PrivateKey {
        Bech32PrivateKey { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

pub struct Bech32AccountPublicKey {
    value: String,
}

impl Bech32AccountPublicKey {
    pub fn new(value: String) -> Bech32AccountPublicKey {
        Bech32AccountPublicKey { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

pub struct Bech32Address {
    value: String,
}

impl Bech32Address {
    pub fn new(value: String) -> Bech32Address {
        Bech32Address { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

pub fn create_private_key(entropy: String, password: String) -> Bech32PrivateKey {
    let mnemonic = Mnemonic::from_phrase(&entropy, Language::English).unwrap();
    let e = mnemonic.entropy();

    let bech32_private_key =
        Bip32PrivateKey::from_bip39_entropy(e, password.as_bytes()).to_bech32();

    Bech32PrivateKey::new(bech32_private_key)
}

pub fn create_bip32_private_key_from_bech32(
    private_key_bech32: &str,
) -> Result<Bip32PrivateKey, String> {
    let result = match Bip32PrivateKey::from_bech32(private_key_bech32) {
        Ok(pk) => Ok(pk),
        Err(err) => Err(err.to_string()),
    };

    result
}

pub fn create_public_account_key(bip32_private_key: Bip32PrivateKey) -> Bech32AccountPublicKey {
    let account_public_key = bip32_private_key
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .to_public()
        .to_bech32();

    Bech32AccountPublicKey::new(account_public_key)
}

pub fn create_bip32_public_key_from_bech32(
    bech32_public_key: &str,
) -> Result<Bip32PublicKey, String> {
    let result = match Bip32PublicKey::from_bech32(bech32_public_key) {
        Ok(pk) => Ok(pk),
        Err(err) => Err(err.to_string()),
    };

    result
}

pub fn derive_address(
    bip32_account_public_key: &Bip32PublicKey,
    change_index: &u32,
    index: &u32,
) -> Result<Bech32Address, String> {
    let result = match bip32_account_public_key
        .derive(*change_index)
        .and_then(|bip32_public_key| bip32_public_key.derive(*index))
    {
        Ok(pbk) => Ok(Bech32Address::new(pbk.to_bech32())),
        Err(err) => Err(err.to_string()),
    };

    result
}

pub fn create_payment_address(
    network: &u8,
    bip32_payment_verification_key: &Bip32PublicKey,
    bip32_stake_verification_key: &Bip32PublicKey,
) -> Result<Bech32Address, String> {
    let payment_v_key_raw_key = bip32_payment_verification_key.to_raw_key();
    let payment_ed25519_key_hash = payment_v_key_raw_key.hash();
    let payment_credential = StakeCredential::from_keyhash(&payment_ed25519_key_hash);

    let stake_v_raw_key = bip32_stake_verification_key.to_raw_key();
    let stake_ed25519_key_hash = stake_v_raw_key.hash();
    let stake_credential = StakeCredential::from_keyhash(&stake_ed25519_key_hash);

    let base_address = BaseAddress::new(*network, &payment_credential, &stake_credential);

    let result = match base_address.to_address().to_bech32(None) {
        Ok(addr) => Ok(Bech32Address::new(addr)),
        Err(err) => Err(err.to_string()),
    };

    result
}
