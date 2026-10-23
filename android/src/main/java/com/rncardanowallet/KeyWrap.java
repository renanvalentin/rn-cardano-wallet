package com.rncardanowallet;

import android.os.Build;

import androidx.annotation.RequiresApi;

import java.util.Base64;
import java.util.UUID;

public class KeyWrap {
  private static final String SEPARATOR = ":";

  @RequiresApi(api = Build.VERSION_CODES.O)
  public static String wrap(String data) throws Exception {
    UUID uuid = UUID.randomUUID();
    String keyName = uuid.toString();

    CryptoService.generateKeyPair(keyName);
    String cipherText = CryptoService.encrypt(keyName, data);

    String result = keyName + SEPARATOR + cipherText;
    return Base64.getEncoder().encodeToString(result.getBytes());
  }

  @RequiresApi(api = Build.VERSION_CODES.O)
  public static String unwrap(String data) throws Exception {
    byte[] decodedBytes = Base64.getDecoder().decode(data);
    String decodedString = new String(decodedBytes);
    String[] payload = decodedString.split(SEPARATOR);
    String keyName = payload[0];
    String cipherTex = payload[1];

    String result = CryptoService.decrypt(keyName, cipherTex);
    return result;
  }
}
