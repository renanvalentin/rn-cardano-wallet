package com.rncardanowallet;

import android.os.Bundle;

public class PublicAccountKeyRequest {
  public static Bundle create(String base64Bip32PrivateKey, String password) {
    Bundle bundle = new Bundle();
    bundle.putString("base64Bip32PrivateKey", base64Bip32PrivateKey);
    bundle.putString("password", password);
    return bundle;
  }
}
