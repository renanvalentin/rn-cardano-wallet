#!/usr/bin/env bash

echo "------------"
echo "Running compile-library.sh"
echo $@

# if [ "$#" -ne 2 ]
# then
#     echo "Usage (note: only call inside xcode!):"
#     echo "compile-library.sh <FFI_TARGET> <buildvariant>"
#     exit 1
# fi

# # what to pass to cargo build -p, e.g. your_lib_ffi
# FFI_TARGET=$1
# # buildvariant from our xcconfigs
# BUILDVARIANT=$2

# remove later

FFI_TARGET=rusty_cardano_wallet
BUILDVARIANT=debug

HAS_CARGO_IN_PATH=`which cargo; echo $?`

if [ "${HAS_CARGO_IN_PATH}" -ne "0" ]; then
    source $HOME/.cargo/env
fi

RELFLAG=
if [[ "$BUILDVARIANT" != "debug" ]]; then
  RELFLAG=--release
fi

set -euvx

if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]; then
  # Assume we're in Xcode, which means we're probably cross-compiling.
  # In this case, we need to add an extra library search path for build scripts and proc-macros,
  # which run on the host instead of the target.
  # (macOS Big Sur does not have linkable libraries in /usr/lib/.)
  export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
fi

IS_SIMULATOR=0
if [ "${LLVM_TARGET_TRIPLE_SUFFIX-}" = "-simulator" ]; then
  IS_SIMULATOR=1
fi

if [ -z "${PODS_TARGET_SRCROOT}" ]; then
    ROOT_DIR="${SRCROOT}"
else
    ROOT_DIR="${PODS_TARGET_SRCROOT}"
fi

export CARGO_PROJECT_DIR="${ROOT_DIR}/rust"
export MANIFEST_PATH="${CARGO_PROJECT_DIR}/Cargo.toml"
# export TARGET_PATH="${PODS_TARGET_SRCROOT}/target"
export TARGET_PATH="${PODS_TARGET_SRCROOT}/build"

rm -rf ${TARGET_PATH}

for arch in $ARCHS; do
  case "$arch" in
    x86_64)
      if [ $IS_SIMULATOR -eq 0 ]; then
        echo "Building for x86_64, but not a simulator build. What's going on?" >&2
        exit 2
      fi

      # Intel iOS simulator
      export CFLAGS_x86_64_apple_ios="-target x86_64-apple-ios"
      cargo build -p $FFI_TARGET --lib $RELFLAG --target x86_64-apple-ios --manifest-path $MANIFEST_PATH
      ;;

    arm64)
      # if [ $IS_SIMULATOR -eq 0 ]; then
        # Hardware iOS targets
        cargo build -p $FFI_TARGET --lib $RELFLAG --target aarch64-apple-ios --manifest-path $MANIFEST_PATH
      # else
        cargo build -p $FFI_TARGET --lib $RELFLAG --target aarch64-apple-ios-sim --manifest-path $MANIFEST_PATH
      # fi
        xcodebuild -create-xcframework \
          -library ${CARGO_PROJECT_DIR}/target/aarch64-apple-ios/debug/librusty_cardano_wallet.a \
          -library ${CARGO_PROJECT_DIR}/target/aarch64-apple-ios-sim/debug/librusty_cardano_wallet.a \
          -output ${TARGET_PATH}/rusty_cardano_wallet.xcframework
  esac
done

mkdir "${TARGET_PATH}"/include/
cp -f "${CARGO_PROJECT_DIR}"/include/*.h "${TARGET_PATH}"/include/
# cp -f "${CARGO_PROJECT_DIR}"/target/"${TARGET_NAME}"/debug/*.a "${CONFIGURATION_BUILD_DIR}"/
