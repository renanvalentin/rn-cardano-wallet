package com.rncardanowalletexample;

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

public class MyService extends Service {
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

          super.handleMessage(msg);
    }
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
