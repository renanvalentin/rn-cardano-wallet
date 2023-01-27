package com.rncardanowallet;

import android.app.Activity;
import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.ServiceConnection;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.os.Messenger;
import android.os.RemoteException;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.annotation.RequiresApi;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.module.annotations.ReactModule;
import com.facebook.react.bridge.Promise;


import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;

import okhttp3.internal.ws.RealWebSocket;

@ReactModule(name = RnCardanoWalletModule.NAME)
public class RnCardanoWalletModule extends NativeRnCardanoWalletSpec {
  public static final String NAME = "RnCardanoWallet";

  @Nullable
  private final ReactApplicationContext mReactApplicationContext;

  public RnCardanoWalletModule(ReactApplicationContext reactContext) {
    super(reactContext);

    mReactApplicationContext = reactContext;
  }
  public static native String nativePrivateKey(String mnemonic, String salt, String password);
  private static native boolean nativeMnemonicValidation(String mnemonic);
  public static native String nativePublicAccountKey(String bip32PrivateKey, String password);
  public static native String nativeBech32Address(String bech32PublicKey, int changeIndex, int index);
  public static native String nativePaymentAddress(String bech32PublicKey, int changeIndex, int index);
  public static native String nativeTransactionBody(String configJson, String inputsJson, String outputJson, String bech32ChangeAddress, double ttl);
  public static native String nativeTransaction(String bip32PrivateKey, String[] paymentSigningKeyPathsJson, String transactionBody);

  private void sendMessage(Message message) throws RemoteException {
    RnCardanoWalletActivityMessenger messenger = (RnCardanoWalletActivityMessenger)mReactApplicationContext.getCurrentActivity();
    messenger.sendMessage(message);
  }
  @RequiresApi(api = Build.VERSION_CODES.O)
  @Override
  public void privateKey(String entropy, String salt, String password, Promise promise) {
    MessageHandler handler = (Message msg) -> {
      Bundle bundle = msg.getData();
      String privateKey = bundle.getString("privateKey");
      try {
        String wrapped = KeyWrap.wrap(privateKey);
        promise.resolve(wrapped);
      } catch (Exception e) {
        promise.resolve(e);
      }
    };

    Bundle request = PrivateKeyRequest.create(entropy, salt, password);

    Message msg = ReplyMessenger.create(request, MessengerService.CREATE_PRIVATE_KEY, handler);

    try {
      this.sendMessage(msg);
    } catch (RemoteException e) {
      promise.reject(e);
    }
  }

  @RequiresApi(api = Build.VERSION_CODES.O)
  @Override
  public void publicAccountKey(String base64Bip32PrivateKey, String password, Promise promise) {
    MessageHandler handler = (Message msg) -> {
      Bundle bundle = msg.getData();
      String publicAccountKey = bundle.getString("publicAccountKey");
      promise.resolve(publicAccountKey);
    };

    String unwrappedKey = null;
    try {
      unwrappedKey = KeyWrap.unwrap(base64Bip32PrivateKey);
    } catch (Exception e) {
      promise.reject(e);
    }

    Bundle request = PublicAccountKeyRequest.create(unwrappedKey, password);

    Message msg = ReplyMessenger.create(request, MessengerService.CREATE_PUBLIC_ACCOUNT_KEY, handler);

    try {
      this.sendMessage(msg);
    } catch (RemoteException e) {
      promise.reject(e);
    }
  }

  @Override
  public void validateMnemonic(String mnemonic, Promise promise) {
    promise.resolve(RnCardanoWalletModule.nativeMnemonicValidation(mnemonic));
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
  public void transaction(String base64Bip32PrivateKey, String password, String paymentSigningKeyPathsJson, String transactionBodyJson, Promise promise) {

  }

  @Override
  @NonNull
  public String getName() {
    return NAME;
  }

  static {
    System.loadLibrary("rusty_cardano_wallet");
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
