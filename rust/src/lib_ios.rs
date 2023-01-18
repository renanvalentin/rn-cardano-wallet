use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::str;
use std::{ptr, slice};

use crate::bip39;
use crate::keygen;
use crate::transactions;

#[repr(C)]
pub struct PrivateKey {
    value: *mut c_char,
}

#[repr(C)]
pub struct PublicAccountKey {
    value: *mut c_char,
}

#[repr(C)]
pub struct MnemonicValidation {
    value: bool,
}

#[repr(C)]
pub struct Bech32Address {
    value: *mut c_char,
}

#[repr(C)]
pub struct PaymentAddress {
    value: *mut c_char,
}

#[repr(C)]
pub struct TransactionBody {
    value: *mut c_char,
}

#[repr(C)]
pub struct Transaction {
    value: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn private_key_create(
    c_entropy: *const c_char,
    c_password: *const c_char,
) -> *mut PrivateKey {
    let entropy = match c_char_to_str(c_entropy) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let password = match c_char_to_str(c_password) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let bech32_private_key = match keygen::create_private_key(&entropy, &password) {
        Ok(pk) => pk,
        Err(_) => return ptr::null_mut(),
    };

    let private_key_str = match CString::new(bech32_private_key.value()) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let private_key = PrivateKey {
        value: private_key_str.into_raw(),
    };

    Box::into_raw(Box::new(private_key))
}

#[no_mangle]
pub unsafe extern "C" fn private_key_free(private_key_ptr: *mut PrivateKey) {
    if private_key_ptr.is_null() {
        return;
    }

    let private_key = &*private_key_ptr;

    if !private_key.value.is_null() {
        drop(CString::from_raw(private_key.value));
    }

    drop(Box::from_raw(private_key_ptr))
}

#[no_mangle]
pub unsafe extern "C" fn public_account_key_create(
    c_bip32_private_key_bytes: *const u8,
    c_bip32_private_key_len: size_t,
) -> *mut PublicAccountKey {
    let byte_slice =
        slice::from_raw_parts(c_bip32_private_key_bytes, c_bip32_private_key_len as usize);

    let bip32_private_key_str = match byte_slice_as_utf8(byte_slice) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let bip32_private_key =
        match keygen::create_bip32_private_key_from_bech32(&bip32_private_key_str) {
            Ok(pk) => pk,
            Err(_) => return ptr::null_mut(),
        };

    let bech32_public_account_key = keygen::create_public_account_key(bip32_private_key);

    let bech32_public_account_key_str = match CString::new(bech32_public_account_key.value()) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let public_account_key = PublicAccountKey {
        value: bech32_public_account_key_str.into_raw(),
    };

    Box::into_raw(Box::new(public_account_key))
}

#[no_mangle]
pub unsafe extern "C" fn public_account_key_free(public_account_key_ptr: *mut PublicAccountKey) {
    if public_account_key_ptr.is_null() {
        return;
    }

    let public_account_key = &*public_account_key_ptr;

    if !public_account_key.value.is_null() {
        drop(CString::from_raw(public_account_key.value));
    }

    drop(Box::from_raw(public_account_key_ptr))
}

#[no_mangle]
pub unsafe extern "C" fn mnemonic_validation_create(
    c_mnemonic: *const c_char,
) -> *mut MnemonicValidation {
    let mnemonic_str = match c_char_to_str(c_mnemonic) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let result = bip39::validate(&mnemonic_str);

    let mnemonic_validation = MnemonicValidation { value: result };

    Box::into_raw(Box::new(mnemonic_validation))
}

#[no_mangle]
pub unsafe extern "C" fn mnemonic_validation_free(
    mnemonic_validation_ptr: *mut MnemonicValidation,
) {
    if mnemonic_validation_ptr.is_null() {
        return;
    }

    drop(Box::from_raw(mnemonic_validation_ptr))
}

#[no_mangle]
pub unsafe extern "C" fn bech32_address_create(
    c_bech32_public_key: *const c_char,
    change_index: u32,
    index: u32,
) -> *mut Bech32Address {
    let bip32_public_key_str = match c_char_to_str(c_bech32_public_key) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let bip32_public_key = match keygen::create_bip32_public_key_from_bech32(&bip32_public_key_str)
    {
        Ok(pk) => pk,
        Err(_) => return ptr::null_mut(),
    };

    let address_derivation = keygen::derive_address(&bip32_public_key, &change_index, &index);

    let bech32_address = match address_derivation {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let bech32_address_str = match CString::new(bech32_address.value()) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let c_bech32_address = Bech32Address {
        value: bech32_address_str.into_raw(),
    };

    Box::into_raw(Box::new(c_bech32_address))
}

#[no_mangle]
pub unsafe extern "C" fn bech32_address_free(bech32_address_ptr: *mut Bech32Address) {
    if bech32_address_ptr.is_null() {
        return;
    }

    let bech32_address = &*bech32_address_ptr;

    if !bech32_address.value.is_null() {
        drop(CString::from_raw(bech32_address.value));
    }

    drop(Box::from_raw(bech32_address_ptr))
}

#[no_mangle]
pub unsafe extern "C" fn payment_address_create(
    network: u8,
    c_bip32_payment_verification_key: *const c_char,
    c_bip32_stake_verification_key: *const c_char,
) -> *mut PaymentAddress {
    let bip32_payment_verification_key_str = match c_char_to_str(c_bip32_payment_verification_key) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let bip32_stake_verification_key_str = match c_char_to_str(c_bip32_stake_verification_key) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let bip32_payment_verification_key =
        match keygen::create_bip32_public_key_from_bech32(&bip32_payment_verification_key_str) {
            Ok(pk) => pk,
            Err(_) => return ptr::null_mut(),
        };

    let bip32_stake_verification_key =
        match keygen::create_bip32_public_key_from_bech32(&bip32_stake_verification_key_str) {
            Ok(pk) => pk,
            Err(_) => return ptr::null_mut(),
        };

    let payment_address_result = match keygen::create_payment_address(
        &network,
        &bip32_payment_verification_key,
        &bip32_stake_verification_key,
    ) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let payment_address_str = match CString::new(payment_address_result.value()) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let payment_address = PaymentAddress {
        value: payment_address_str.into_raw(),
    };

    Box::into_raw(Box::new(payment_address))
}

#[no_mangle]
pub unsafe extern "C" fn payment_address_free(payment_address_ptr: *mut PaymentAddress) {
    if payment_address_ptr.is_null() {
        return;
    }

    let payment_address = &*payment_address_ptr;

    if !payment_address.value.is_null() {
        drop(CString::from_raw(payment_address.value));
    }

    drop(Box::from_raw(payment_address_ptr))
}

#[no_mangle]
pub unsafe extern "C" fn transaction_body_create(
    c_config_json: *const c_char,
    c_inputs_json: *const c_char,
    c_output_json: *const c_char,
    c_bech32_change_address: *const c_char,
    ttl: u64,
) -> *mut TransactionBody {
    let config_json = match c_char_to_str(c_config_json) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let inputs_json = match c_char_to_str(c_inputs_json) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let output_json = match c_char_to_str(c_output_json) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let bech32_change_address = match c_char_to_str(c_bech32_change_address) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let transaction_body = match transactions::create_transaction_body(
        &config_json,
        &inputs_json,
        &output_json,
        &bech32_change_address,
        ttl,
    ) {
        Ok(t) => t,
        Err(err) => {
            return ptr::null_mut();
        }
    };

    let json = match transaction_body.to_json() {
        Ok(json) => json,
        Err(_) => return ptr::null_mut(),
    };

    let json_str = match CString::new(json) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let transaction_body = TransactionBody {
        value: json_str.into_raw(),
    };

    Box::into_raw(Box::new(transaction_body))
}

#[no_mangle]
pub unsafe extern "C" fn transaction_body_free(transaction_body_ptr: *mut TransactionBody) {
    if transaction_body_ptr.is_null() {
        return;
    }

    let transaction_body = &*transaction_body_ptr;

    if !transaction_body.value.is_null() {
        drop(CString::from_raw(transaction_body.value));
    }

    drop(Box::from_raw(transaction_body_ptr))
}

#[no_mangle]
pub unsafe extern "C" fn transaction_create(
    c_bip32_private_key_bytes: *const u8,
    c_bip32_private_key_len: size_t,
    c_payment_signing_key_paths_json: *const c_char,
    c_transaction_body_json: *const c_char,
) -> *mut Transaction {
    let byte_slice =
        slice::from_raw_parts(c_bip32_private_key_bytes, c_bip32_private_key_len as usize);

    let bip32_private_key_str = match byte_slice_as_utf8(byte_slice) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let payment_signing_key_paths_json = match c_char_to_str(c_payment_signing_key_paths_json) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let transaction_body_json = match c_char_to_str(c_transaction_body_json) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let bip32_private_key =
        match keygen::create_bip32_private_key_from_bech32(&bip32_private_key_str) {
            Ok(pk) => pk,
            Err(_) => return ptr::null_mut(),
        };

    let transaction_hex = match transactions::create_transaction(
        &bip32_private_key,
        &payment_signing_key_paths_json,
        &transaction_body_json,
    ) {
        Ok(t) => t,
        Err(err) => {
            return ptr::null_mut();
        }
    };

    let transaction_hex_cstring = match CString::new(transaction_hex) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let transaction = Transaction {
        value: transaction_hex_cstring.into_raw(),
    };

    Box::into_raw(Box::new(transaction))
}

#[no_mangle]
pub unsafe extern "C" fn transaction_free(transaction_ptr: *mut Transaction) {
    if transaction_ptr.is_null() {
        return;
    }

    let transaction = &*transaction_ptr;

    if !transaction.value.is_null() {
        drop(CString::from_raw(transaction.value));
    }

    drop(Box::from_raw(transaction_ptr))
}

enum ErrorType {
    NullPtr,
    InvalidData,
}

fn byte_slice_as_utf8(bytes: &[u8]) -> Result<&str, String> {
    match str::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(err) => Err(format!("invalid UTF-8 data: {}", err)),
    }
}

fn c_char_to_str(c_char: *const c_char) -> Result<String, ErrorType> {
    let raw = unsafe {
        if c_char.is_null() {
            return Err(ErrorType::NullPtr);
        }

        CStr::from_ptr(c_char)
    };

    let url_as_str = match raw.to_str() {
        Ok(s) => s,
        Err(_) => return Err(ErrorType::InvalidData),
    };

    Ok(url_as_str.to_string())
}
