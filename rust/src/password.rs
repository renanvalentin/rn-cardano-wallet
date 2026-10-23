extern crate cryptoxide;

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use cryptoxide::{chacha20poly1305::ChaCha20Poly1305, hmac::Hmac, pbkdf2::pbkdf2, sha2::Sha512};

use self::password_encryption_parameter::{NONCE_SIZE, SALT_SIZE};

mod password_encryption_parameter {
    pub const ITER: u32 = 19_162;
    pub const SALT_SIZE: usize = 32;
    pub const NONCE_SIZE: usize = 12;
    pub const KEY_SIZE: usize = 32;
    pub const TAG_SIZE: usize = 16;

    pub const METADATA_SIZE: usize = SALT_SIZE + NONCE_SIZE + TAG_SIZE;

    pub const SALT_START: usize = 0;
    pub const SALT_END: usize = SALT_START + SALT_SIZE;
    pub const NONCE_START: usize = SALT_END;
    pub const NONCE_END: usize = NONCE_START + NONCE_SIZE;
    pub const TAG_START: usize = NONCE_END;
    pub const TAG_END: usize = TAG_START + TAG_SIZE;
    pub const ENCRYPTED_START: usize = TAG_END;
}

/// encrypt the given data with a password, a salt and a nonce
///
/// Salt: must be 32 bytes long;
/// Nonce: must be 12 bytes long;
///
pub fn password_encrypt(password: &str, data: &[u8]) -> Result<String, String> {
    let mut salt = [0u8; SALT_SIZE];
    getrandom::getrandom(&mut salt);

    let mut nonce = [0u8; NONCE_SIZE];
    getrandom::getrandom(&mut salt);

    let password = password.as_bytes();
    if salt.len() != password_encryption_parameter::SALT_SIZE {
        return Err(String::from("Invalid Salt Size, expected 32 bytes"));
    }
    if nonce.len() != password_encryption_parameter::NONCE_SIZE {
        return Err(String::from("Invalid Nonce Size, expected 12 bytes"));
    }

    let key = {
        let mut mac = Hmac::new(Sha512::new(), &password);
        let mut key = [0u8; password_encryption_parameter::KEY_SIZE];
        pbkdf2(
            &mut mac,
            &salt[..],
            password_encryption_parameter::ITER,
            &mut key,
        );
        key
    };

    let mut tag = [0u8; password_encryption_parameter::TAG_SIZE];
    let mut encrypted: Vec<u8> = std::iter::repeat(0).take(data.len()).collect();
    {
        ChaCha20Poly1305::new(&key, &nonce, &[]).encrypt(&data, &mut encrypted, &mut tag);
    }

    let mut output = Vec::with_capacity(data.len() + password_encryption_parameter::METADATA_SIZE);
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce);
    output.extend_from_slice(&tag);
    output.extend_from_slice(&encrypted);

    Ok(general_purpose::STANDARD.encode(&output))
}

/// decrypt the data with the password
///
pub fn password_decrypt(password: &str, encrypted_data: &[u8]) -> Result<String, String> {
    let encrypted_data = match general_purpose::STANDARD.decode(&encrypted_data) {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid Base64: {}", e)),
    };

    if encrypted_data.len() <= password_encryption_parameter::METADATA_SIZE {
        return Err(String::from("Not enough data to decrypt"));
    }

    let password = password.as_bytes();
    let salt = &encrypted_data
        [password_encryption_parameter::SALT_START..password_encryption_parameter::SALT_END];
    let nonce = &encrypted_data
        [password_encryption_parameter::NONCE_START..password_encryption_parameter::NONCE_END];
    let tag = &encrypted_data
        [password_encryption_parameter::TAG_START..password_encryption_parameter::TAG_END];
    let encrypted = &encrypted_data[password_encryption_parameter::ENCRYPTED_START..];

    let key = {
        let mut mac = Hmac::new(Sha512::new(), &password);
        let mut key = [0u8; password_encryption_parameter::KEY_SIZE];
        pbkdf2(
            &mut mac,
            &salt[..],
            password_encryption_parameter::ITER,
            &mut key,
        );
        key
    };

    let mut decrypted: Vec<u8> = std::iter::repeat(0).take(encrypted.len()).collect();
    let decryption_succeed =
        { ChaCha20Poly1305::new(&key, &nonce, &[]).decrypt(&encrypted, &mut decrypted, &tag) };

    if decryption_succeed {
        match String::from_utf8(decrypted) {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e)),
        }
    } else {
        Err(String::from("Cannot decrypt the data"))
    }
}
