use jni::objects::{JClass, JString};
use jni::sys::{jint, jlong, jobject, jstring};
use jni::JNIEnv;

use crate::keygen;

#[no_mangle]
pub extern "system" fn Java_com_rncardanowallet_RnCardanoWalletModule_nativePrivateKey(
    env: JNIEnv,
    class: JClass,
    mnemonic: JString,
    password: JString,
) -> jobject {
    let mnemonic: String = env
        .get_string(mnemonic)
        .expect("Couldn't get java string!")
        .into();

    let password: String = env
        .get_string(password)
        .expect("Couldn't get java string!")
        .into();

    let private_key = match keygen::create_private_key(&mnemonic, &password) {
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

// #[no_mangle]
// pub extern "system" fn Java_com_rncardanowallet_RnCardanoWalletModule_nativeBech32Address(
//     env: JNIEnv,
//     class: JClass,
//     bech32_public_account_key: JString,
//     change_index: jlong,
//     index: jint,
// ) -> jobject {
//     let bech32_public_account_key: String = env
//         .get_string(bech32_public_account_key)
//         .expect("Couldn't get java string!")
//         .into();

//     let bip32_public_key =
//         match keygen::create_bip32_public_key_from_bech32(&bech32_public_account_key) {
//             Ok(pk) => pk,
//             Err(err) => {
//                 env.throw_new("java/lang/RuntimeException", "panic occurred");
//                 return ::std::ptr::null_mut() as jstring;
//             }
//         };

//     let change_index = match u32::try_from(change_index) {
//         Ok(index) => index,
//         Err(err) => {
//             env.throw_new("java/lang/RuntimeException", "panic occurred");
//             return ::std::ptr::null_mut() as jstring;
//         }
//     };

//     let index = match u32::try_from(index) {
//         Ok(index) => index,
//         Err(err) => {
//             env.throw_new("java/lang/RuntimeException", "panic occurred");
//             return ::std::ptr::null_mut() as jstring;
//         }
//     };

//     let address_derivation = match keygen::derive_address(&bip32_public_key, &change_index, &index)
//     {
//         Ok(address_derivation) => address_derivation,
//         Err(err) => {
//             env.throw_new("java/lang/RuntimeException", "panic occurred");
//             return ::std::ptr::null_mut() as jstring;
//         }
//     };

//     let result = match env.new_string(address_derivation.value()) {
//         Ok(result) => result,
//         Err(err) => {
//             env.throw_new("java/lang/RuntimeException", "panic occurred");
//             return ::std::ptr::null_mut() as jstring;
//         }
//     };

//     result.into_raw()
// }
