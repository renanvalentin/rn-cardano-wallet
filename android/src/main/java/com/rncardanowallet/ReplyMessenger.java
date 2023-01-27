package com.rncardanowallet;

import android.os.Bundle;
import android.os.Message;
import android.os.Messenger;

public class ReplyMessenger {
  public static Message create(Bundle requestPayload, int what, MessageHandler handler) {
    Messenger replyMessenger = new Messenger(new HandlerReplyMsg(handler));

    Message msg = Message.obtain(null, what );
    msg.setData(requestPayload);
    msg.replyTo = replyMessenger;

    return msg;
  }
}
