package com.rncardanowallet;

import android.os.Build;
import android.security.keystore.KeyGenParameterSpec;
import android.security.keystore.KeyInfo;
import android.security.keystore.KeyProperties;
import android.util.Log;
import androidx.annotation.RequiresApi;
import java.nio.charset.StandardCharsets;
import java.security.Key;
import java.security.KeyFactory;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.KeyStore;
import java.security.PublicKey;
import java.security.Signature;
import java.security.spec.X509EncodedKeySpec;
import java.util.Base64;

import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;
import javax.crypto.spec.IvParameterSpec;

// Credits: https://github.dev/lauhon/WalletPOC/blob/5e2edfbc798fc28d14696276e991fafb5c87754c/packages/react-native-secure-encryption-module/android/src/main/java/com/reactnativesecureencryptionmodule/service/EncryptionService.java#L18
public class EncryptionService {

  private static final String TAG = "EncryptionService";
  private static final String IV_SEPARATOR = "]";

  private static final String AES_MODE = KeyProperties.KEY_ALGORITHM_AES
    + "/" + KeyProperties.BLOCK_MODE_CBC
    + "/" + KeyProperties.ENCRYPTION_PADDING_PKCS7;


  @RequiresApi(api = Build.VERSION_CODES.N)
  private static Key loadKey(String alias) throws Exception {
    try {
      KeyStore keyStore = KeyStore.getInstance("AndroidKeyStore");
      keyStore.load(null);

      return keyStore.getKey(alias, null);
    } catch (Exception e) {
      throw new Exception("Key not found");
    }
  }

  @RequiresApi(api = Build.VERSION_CODES.O)
  public static SecretKey generateKeyPair(String keyAlias) throws Exception {
    try {
      KeyGenParameterSpec keyGenParameterSpec = new KeyGenParameterSpec.Builder(keyAlias,
        KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT)
        .setBlockModes(KeyProperties.BLOCK_MODE_CBC)
        .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_PKCS7)
        .setRandomizedEncryptionRequired(true)
        .build();

      KeyGenerator keyGenerator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES,
        "AndroidKeyStore");
      keyGenerator.init(keyGenParameterSpec);

      SecretKey secretKey = keyGenerator.generateKey();

      return secretKey;
    } catch (Exception e) {
      throw new Exception("Error while creating Keypair");
    }
  }

  @RequiresApi(api = Build.VERSION_CODES.O)
  public boolean deleteKeyPair(String alias) throws Exception {
    try {
      KeyStore keyStore = KeyStore.getInstance("AndroidKeyStore");
      keyStore.load(null);
      keyStore.deleteEntry(alias);

      return true;
    } catch (Exception e) {
      throw new Exception("Key not found");
    }
  }


  @RequiresApi(api = Build.VERSION_CODES.O)
  public static String encrypt(String keyName, String message) throws Exception {
    Key key= loadKey(keyName);

    Cipher cipher = Cipher.getInstance(AES_MODE);
    cipher.init(Cipher.ENCRYPT_MODE, key);

    byte[] encryptedBytes = cipher.doFinal(message.getBytes());
    byte[] iv = cipher.getIV();

    return Base64.getEncoder().encodeToString(encryptedBytes) + IV_SEPARATOR + Base64.getEncoder().encodeToString(iv);
  }

  @RequiresApi(api = Build.VERSION_CODES.O)
  public static String decrypt(String keyName, String cipherText) throws Exception {
    Key key = loadKey(keyName);
    String[] payload = cipherText.split(IV_SEPARATOR);
    byte[] input = Base64.getDecoder().decode(payload[0]);
    byte[] iv = Base64.getDecoder().decode(payload[1]);

    IvParameterSpec ivSpec = new IvParameterSpec(iv);

    Cipher cipher = Cipher.getInstance(AES_MODE);
    cipher.init(Cipher.DECRYPT_MODE, key, ivSpec);

    byte[] result = cipher.doFinal(input);

    return new String(result);
  }
}
