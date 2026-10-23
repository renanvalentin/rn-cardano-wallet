package com.rncardanowallet;

import android.app.Service;
import android.content.Context;
import android.content.Intent;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.os.Messenger;

import androidx.annotation.RequiresApi;

import java.util.Base64;
import java.util.UUID;

public class MessengerService extends Service {
  private static final String SEPARATOR = ":";

  /**
   * Command to the service to display a message
   */

  static final int CREATE_PRIVATE_KEY = 2;
  static final int CREATE_PRIVATE_KEY_REPLY = 21;
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
            MessengerService.createPrivateKey(msg.getData(), msg.replyTo);
          } catch (Exception e) {
            throw new RuntimeException(e);
          }

          break;
        case CREATE_PUBLIC_ACCOUNT_KEY:
          try {
            MessengerService.createPublicAccountKey(msg.getData(), msg.replyTo);
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
  private static void createPrivateKey(Bundle request, Messenger replyMessenger) throws Exception {
   // android.os.Debug.waitForDebugger();
    String mnemonic = request.getString("mnemonic");
    String salt = request.getString("salt");
    String password = request.getString("password");

    final String privateKey = RnCardanoWalletModule.nativePrivateKey(mnemonic, salt, password);

    request.putString("privateKey", privateKey);

    Message message = new Message();
    message.setData(request);

    replyMessenger.send(message);
  }

  @RequiresApi(api = Build.VERSION_CODES.O)
  private static void createPublicAccountKey(Bundle request, Messenger replyMessenger) throws Exception {
    String base64Bip32PrivateKey = request.getString("base64Bip32PrivateKey");
    String password = request.getString("password");

    final String publicAccountKey = RnCardanoWalletModule.nativePublicAccountKey(base64Bip32PrivateKey, password);

    request.putString("publicAccountKey", publicAccountKey);

    Message message = new Message();
    message.setData(request);

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
    mMessenger = new Messenger(new IncomingHandler(this));
    return mMessenger.getBinder();
  }

}
