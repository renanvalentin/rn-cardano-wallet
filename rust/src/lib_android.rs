use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jbyte, jint, jlong, jobject, jstring};
use jni::JNIEnv;

use crate::keygen;
use crate::{bip39, transactions};

#[no_mangle]
pub extern "system" fn Java_com_rncardanowallet_RnCardanoWalletModule_nativePrivateKey(
    env: JNIEnv,
    _: JClass,
    mnemonic: JString,
    salt: JString,
    password: JString,
) -> jobject {
    let mnemonic: String = env
        .get_string(mnemonic)
        .expect("Couldn't get java string!")
        .into();

    let salt: String = env
        .get_string(salt)
        .expect("Couldn't get java string!")
        .into();

    let password: String = env
        .get_string(password)
        .expect("Couldn't get java string!")
        .into();

    let private_key = match keygen::create_private_key(&mnemonic, &salt, &password) {
        Ok(private_key) => private_key,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let result = match env.new_string(private_key.value()) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    result.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_rncardanowallet_RnCardanoWalletModule_nativePublicAccountKey(
    env: JNIEnv,
    _: JClass,
    bip32_private_key: JString,
    password: JString,
) -> jobject {
    let bip32_private_key: String = env
        .get_string(bip32_private_key)
        .expect("Couldn't get java string!")
        .into();

    let password: String = env
        .get_string(password)
        .expect("Couldn't get java string!")
        .into();

    let bip32_private_key =
        match keygen::create_bip32_private_key_from_bech32(&bip32_private_key, &password) {
            Ok(pk) => pk,
            Err(err) => {
                env.throw_new("java/lang/RuntimeException", "panic occurred");
                return ::std::ptr::null_mut() as jstring;
            }
        };

    let bech32_public_account_key = keygen::create_public_account_key(bip32_private_key);

    let result = match env.new_string(bech32_public_account_key.value()) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    result.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_rncardanowallet_RnCardanoWalletModule_nativeMnemonicValidation(
    env: JNIEnv,
    _: JClass,
    mnemonic: JString,
) -> jboolean {
    let mnemonic: String = env
        .get_string(mnemonic)
        .expect("Couldn't get java string!")
        .into();

    let result = bip39::validate(&mnemonic);

    u8::from(result) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_rncardanowallet_RnCardanoWalletModule_nativeBech32Address(
    env: JNIEnv,
    _: JClass,
    bech32_public_key: JString,
    change_index: jint,
    index: jint,
) -> jstring {
    let bech32_public_key: String = env
        .get_string(bech32_public_key)
        .expect("Couldn't get java string!")
        .into();

    let change_index = match u32::try_from(change_index) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let index = match u32::try_from(index) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let bip32_public_key = match keygen::create_bip32_public_key_from_bech32(&bech32_public_key) {
        Ok(pk) => pk,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let address_derivation = keygen::derive_address(&bip32_public_key, &change_index, &index);

    let bech32_address = match address_derivation {
        Ok(s) => s,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let result = match env.new_string(bech32_address.value()) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    result.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_rncardanowallet_RnCardanoWalletModule_nativePaymentAddress(
    env: JNIEnv,
    _: JClass,
    network: jbyte,
    bip32_payment_verification_key: JString,
    bip32_stake_verification_key: JString,
) -> jstring {
    let bip32_payment_verification_key: String = env
        .get_string(bip32_payment_verification_key)
        .expect("Couldn't get java string!")
        .into();

    let bip32_stake_verification_key: String = env
        .get_string(bip32_stake_verification_key)
        .expect("Couldn't get java string!")
        .into();

    let network = match u8::try_from(network) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let bip32_payment_verification_key =
        match keygen::create_bip32_public_key_from_bech32(&bip32_payment_verification_key) {
            Ok(pk) => pk,
            Err(err) => {
                env.throw_new("java/lang/RuntimeException", "panic occurred");
                return ::std::ptr::null_mut() as jstring;
            }
        };

    let bip32_stake_verification_key =
        match keygen::create_bip32_public_key_from_bech32(&bip32_stake_verification_key) {
            Ok(pk) => pk,
            Err(err) => {
                env.throw_new("java/lang/RuntimeException", "panic occurred");
                return ::std::ptr::null_mut() as jstring;
            }
        };

    let payment_address_result = match keygen::create_payment_address(
        &network,
        &bip32_payment_verification_key,
        &bip32_stake_verification_key,
    ) {
        Ok(s) => s,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let result = match env.new_string(payment_address_result.value()) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    result.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_rncardanowallet_RnCardanoWalletModule_nativeTransactionBody(
    env: JNIEnv,
    _: JClass,
    config_json: JString,
    inputs_json: JString,
    output_json: JString,
    bech32_change_address: JString,
    ttl: jlong,
) -> jstring {
    let config_json: String = env
        .get_string(config_json)
        .expect("Couldn't get java string!")
        .into();

    let inputs_json: String = env
        .get_string(inputs_json)
        .expect("Couldn't get java string!")
        .into();

    let output_json: String = env
        .get_string(output_json)
        .expect("Couldn't get java string!")
        .into();

    let bech32_change_address: String = env
        .get_string(bech32_change_address)
        .expect("Couldn't get java string!")
        .into();

    let ttl = match u64::try_from(ttl) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
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
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let json = match transaction_body.to_json() {
        Ok(json) => json,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let result = match env.new_string(json) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    result.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_rncardanowallet_RnCardanoWalletModule_nativeTransaction(
    env: JNIEnv,
    _: JClass,
    private_key: JString,
    password: JString,
    payment_signing_key_paths_json: JString,
    transaction_body_json: JString,
) -> jstring {
    let private_key: String = env
        .get_string(private_key)
        .expect("Couldn't get java string!")
        .into();

    let password: String = env
        .get_string(password)
        .expect("Couldn't get java string!")
        .into();

    let payment_signing_key_paths_json: String = env
        .get_string(payment_signing_key_paths_json)
        .expect("Couldn't get java string!")
        .into();

    let transaction_body_json: String = env
        .get_string(transaction_body_json)
        .expect("Couldn't get java string!")
        .into();

    let bip32_private_key =
        match keygen::create_bip32_private_key_from_bech32(&private_key, &password) {
            Ok(pk) => pk,
            Err(err) => {
                env.throw_new("java/lang/RuntimeException", "panic occurred");
                return ::std::ptr::null_mut() as jstring;
            }
        };

    let transaction_hex = match transactions::create_transaction(
        &bip32_private_key,
        &payment_signing_key_paths_json,
        &transaction_body_json,
    ) {
        Ok(t) => t,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    let result = match env.new_string(transaction_hex) {
        Ok(result) => result,
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", "panic occurred");
            return ::std::ptr::null_mut() as jstring;
        }
    };

    result.into_raw()
}
