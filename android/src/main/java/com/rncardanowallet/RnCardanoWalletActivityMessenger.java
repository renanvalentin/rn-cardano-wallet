package com.rncardanowallet;

import android.os.Message;
import android.os.RemoteException;

public interface RnCardanoWalletActivityMessenger {
  public void sendMessage(Message message) throws RemoteException;
}
