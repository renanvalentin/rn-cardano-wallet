#!/usr/bin/env bash

echo "------------"
echo "Running compile-library.sh"
echo $@

echo "$PWD"

FFI_TARGET=rusty_cardano_wallet
BUILDVARIANT=debug
RELFLAG=

export ROOT_PATH="/Users/$USER/projects/hack/rn-cardano-wallet"
export CARGO_PROJECT_DIR="${ROOT_PATH}/rust"
export MANIFEST_PATH="${CARGO_PROJECT_DIR}/Cargo.toml"
# export TARGET_PATH="${PODS_TARGET_SRCROOT}/target"
export TARGET_PATH="${ROOT_PATH}/build"
export ANDROID_HOME=/Users/$USER/Library/Android/sdk
export NDK_HOME=$ANDROID_HOME/ndk-bundle

rm -rf ${TARGET_PATH}


cargo build -p $FFI_TARGET --lib $RELFLAG --target aarch64-apple-ios --manifest-path $MANIFEST_PATH
cargo build -p $FFI_TARGET --lib $RELFLAG --target aarch64-apple-ios-sim --manifest-path $MANIFEST_PATH

xcodebuild -create-xcframework \
  -library ${CARGO_PROJECT_DIR}/target/aarch64-apple-ios/${BUILDVARIANT}/librusty_cardano_wallet.a \
  -library ${CARGO_PROJECT_DIR}/target/aarch64-apple-ios-sim/${BUILDVARIANT}/librusty_cardano_wallet.a \
  -output ${TARGET_PATH}/rusty_cardano_wallet.xcframework


mkdir "${TARGET_PATH}"/include/
cp -f "${CARGO_PROJECT_DIR}"/include/*.h "${TARGET_PATH}"/include/
# cp -r "${CARGO_PROJECT_DIR}"/target/cxxbridge "${TARGET_PATH}"/
