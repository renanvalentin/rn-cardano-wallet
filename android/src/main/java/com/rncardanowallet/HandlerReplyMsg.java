package com.rncardanowallet;

import android.os.Handler;
import android.os.Message;

import com.rncardanowallet.MessageHandler;

class HandlerReplyMsg extends Handler {
  MessageHandler mHandler;

  public HandlerReplyMsg(MessageHandler handler) {
  mHandler = handler;
  }

  @Override
  public void handleMessage(Message msg) {
    super.handleMessage(msg);
    mHandler.op(msg);
  }
}
