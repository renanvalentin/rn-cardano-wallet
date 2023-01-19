package com.rncardanowallet;

public class PublicAccountKeyRequest {
  public  String base64Bip32PrivateKey;

  public PublicAccountKeyRequest(String base64Bip32PrivateKey) {
    this.base64Bip32PrivateKey = base64Bip32PrivateKey;
  }
}
