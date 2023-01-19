package com.rncardanowallet;

import android.app.Service;
import android.content.Context;
import android.content.Intent;
import android.os.Build;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.os.Messenger;
import android.widget.Toast;

import androidx.annotation.RequiresApi;

import java.util.Base64;
import java.util.UUID;

public class MessengerService extends Service {
  private static final String SEPARATOR = ":";

  /**
   * Command to the service to display a message
   */

  static final int CREATE_PRIVATE_KEY = 2;
  static final int CREATE_PUBLIC_ACCOUNT_KEY = 3;
  static final int CREATE_TRANSACTION = 4;

  /**
   * Handler of incoming messages from clients.
   */
  static class IncomingHandler extends Handler {
    private Context applicationContext;

    IncomingHandler(Context context) {
      applicationContext = context.getApplicationContext();
    }

    @RequiresApi(api = Build.VERSION_CODES.O)
    @Override
    public void handleMessage(Message msg) {
      switch (msg.what) {
        case CREATE_PRIVATE_KEY:
          try {
            MessengerService.createPrivateKey((PrivateKeyRequest)msg.obj, msg.replyTo);
          } catch (Exception e) {
            throw new RuntimeException(e);
          }

          break;
        case CREATE_PUBLIC_ACCOUNT_KEY:
          try {
            MessengerService.createPublicAccountKey((PublicAccountKeyRequest)msg.obj, msg.replyTo);
          } catch (Exception e) {
            throw new RuntimeException(e);
          }
          break;
        default:
          super.handleMessage(msg);
      }
    }
  }

  @RequiresApi(api = Build.VERSION_CODES.O)
  private static void createPrivateKey(PrivateKeyRequest request, Messenger replyMessenger) throws Exception {
    final String privateKey = RnCardanoWalletModule.nativePrivateKey(request.mnemonic, request.password);

    UUID uuid = UUID.randomUUID();
    String keyName = uuid.toString();

    EncryptionService.generateKeyPair(keyName);
    String cipherText = EncryptionService.encrypt(keyName, privateKey);

    String result = keyName + SEPARATOR + cipherText;

    Message message = new Message();
    message.obj = Base64.getEncoder().encodeToString(result.getBytes());

    replyMessenger.send(message);
  }

  @RequiresApi(api = Build.VERSION_CODES.O)
  private static void createPublicAccountKey(PublicAccountKeyRequest request, Messenger replyMessenger) throws Exception {
    byte[] decodedBytes = Base64.getDecoder().decode(request.base64Bip32PrivateKey);
    String decodedString = new String(decodedBytes);
    String[] payload = decodedString.split(SEPARATOR);
    String keyName = payload[0];
    String cipherTex =  payload[1];

    String privateKey = EncryptionService.decrypt(keyName, cipherTex);

    final String publicAccountKey = RnCardanoWalletModule.nativePublicAccountKey(privateKey);

    Message message = new Message();
    message.obj = publicAccountKey;

    replyMessenger.send(message);
  }


  /**
   * Target we publish for clients to send messages to IncomingHandler.
   */
  Messenger mMessenger;

  /**
   * When binding to the service, we return an interface to our messenger
   * for sending messages to the service.
   */
  @Override
  public IBinder onBind(Intent intent) {
    Toast.makeText(getApplicationContext(), "binding", Toast.LENGTH_SHORT).show();
    mMessenger = new Messenger(new IncomingHandler(this));
    return mMessenger.getBinder();
  }

}
