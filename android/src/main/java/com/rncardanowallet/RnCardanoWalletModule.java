package com.rncardanowallet;

import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.ServiceConnection;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.os.Messenger;
import android.os.RemoteException;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

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

    startService();
    bindService();
  }

  /** Messenger for communicating with the service. */
  Messenger mService = null;

  /** Flag indicating whether we have called bind on the service. */
  boolean bound;

  private ServiceConnection mConnection = new ServiceConnection() {
    public void onServiceConnected(ComponentName className, IBinder service) {
      mService = new Messenger(service);
      bound = true;
    }

    public void onServiceDisconnected(ComponentName className) {
      mService = null;
      bound = false;
    }
  };

  private void startService() {
    Intent intent = new Intent(this.mReactApplicationContext, MessengerService.class);
    this.mReactApplicationContext.startService(intent);
  }

  private void bindService() {
    Intent intent = new Intent(this.mReactApplicationContext, MessengerService.class);
    this.mReactApplicationContext.bindService(intent, mConnection, Context.BIND_AUTO_CREATE);
  }

  public static native String nativePrivateKey(String mnemonic, String password);
  private static native boolean nativeMnemonicValidation(String mnemonic);
  public static native String nativePublicAccountKey(String bip32PrivateKey);
  public static native String nativeBech32Address(String bech32PublicKey, int changeIndex, int index);
  public static native String nativePaymentAddress(String bech32PublicKey, int changeIndex, int index);
  public static native String nativeTransactionBody(String configJson, String inputsJson, String outputJson, String bech32ChangeAddress, double ttl);
  public static native String nativeTransaction(String bip32PrivateKey, String[] paymentSigningKeyPathsJson, String transactionBody);

  @Override
  public void privateKey(String entropy, String password, Promise promise) {
    MessageHandler handler = (Message msg) -> {
      String privateKey = msg.obj.toString();
      promise.resolve(privateKey);
    };

    PrivateKeyRequest request = new PrivateKeyRequest(entropy, password);

    Message msg = ReplyMessenger.create(request, MessengerService.CREATE_PRIVATE_KEY, handler);

    try {
      mService.send(msg);
    } catch (RemoteException e) {
      throw new RuntimeException(e);
    }
  }

  @Override
  public void publicAccountKey(String base64Bip32PrivateKey, Promise promise) {
    MessageHandler handler = (Message msg) -> {
      String publicAccountKey = msg.obj.toString();
      promise.resolve(publicAccountKey);
    };

    PublicAccountKeyRequest request = new PublicAccountKeyRequest(base64Bip32PrivateKey);

    Message msg = ReplyMessenger.create(request, MessengerService.CREATE_PUBLIC_ACCOUNT_KEY, handler);

    try {
      mService.send(msg);
    } catch (RemoteException e) {
      throw new RuntimeException(e);
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
  public void transaction(String base64Bip32PrivateKey, String paymentSigningKeyPathsJson, String transactionBodyJson, Promise promise) {

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
