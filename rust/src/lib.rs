mod bip39;
mod keygen;
mod password;
mod transactions;

#[cfg(target_os = "ios")]
mod lib_ios;

#[cfg(target_os = "ios")]
pub use self::lib_ios::*;

//#[cfg(target_os = "android")]
extern crate jni;
//#[cfg(target_os = "android")]
mod lib_android;
//#[cfg(target_os = "android")]
pub use self::lib_android::*;
