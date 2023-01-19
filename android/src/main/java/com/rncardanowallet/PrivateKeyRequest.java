package com.rncardanowallet;

public class PrivateKeyRequest {
  public  String mnemonic;
  public  String password;

  public PrivateKeyRequest(String mnemonic, String password) {
    this.mnemonic = mnemonic;
    this.password = password;
  }
}
