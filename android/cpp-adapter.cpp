#include <jni.h>
#include "rn-cardano-wallet.h"

extern "C"
JNIEXPORT jint JNICALL
Java_com_rncardanowallet_RnCardanoWalletModule_nativeMultiply(JNIEnv *env, jclass type, jdouble a, jdouble b) {
    return rncardanowallet::multiply(a, b);
}
