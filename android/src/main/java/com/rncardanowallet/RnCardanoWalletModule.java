package com.rncardanowallet;

import androidx.annotation.NonNull;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.module.annotations.ReactModule;

@ReactModule(name = RnCardanoWalletModule.NAME)
public class RnCardanoWalletModule extends NativeRnCardanoWalletSpec {
  public static final String NAME = "RnCardanoWallet";

  public RnCardanoWalletModule(ReactApplicationContext reactContext) {
    super(reactContext);
  }

  @Override
  public void privateKey(String entropy, String password, Promise promise) {

  }

  @Override
  public void publicAccountKey(String base64Bip32PrivateKey, Promise promise) {

  }

  @Override
  public void validateMnemonic(String mnemonic, Promise promise) {

  }

  @Override
  public void bech32Address(String bech32PublicAccountKey, double changeIndex, double index, Promise promise) {

  }

  @Override
  public void paymentAddress(double network, String bech32PaymentVerificationKey, String bech32StakeVerificationKey, Promise promise) {

  }

  @Override
  public void transactionBody(String configJson, String inputsJson, String outputJson, String bech32ChangeAddress, double ttl, Promise promise) {

  }

  @Override
  public void transaction(String base64Bip32PrivateKey, String paymentSigningKeyPathsJson, String transactionBodyJson, Promise promise) {

  }

  @Override
  @NonNull
  public String getName() {
    return NAME;
  }

  static {
    System.loadLibrary("cpp");
  }

//  private static native double nativeMultiply(double a, double b);
//
//  // Example method
//  // See https://reactnative.dev/docs/native-modules-android
//  @Override
//  public double multiply(double a, double b) {
//    return nativeMultiply(a, b);
//  }
}
