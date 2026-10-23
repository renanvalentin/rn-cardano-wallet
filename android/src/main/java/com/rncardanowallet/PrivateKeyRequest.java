package com.rncardanowallet;

import android.os.Bundle;

public class PrivateKeyRequest {
  public static Bundle create(String mnemonic, String salt, String password) {
    Bundle bundle = new Bundle();
    bundle.putString("mnemonic", mnemonic);
    bundle.putString("salt", salt);
    bundle.putString("password", password);
    return bundle;
  }
}
