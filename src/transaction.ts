import type { PrivateKey } from './private-key';
import RnCardanoWallet from './NativeRnCardanoWallet';
import type { TransactionBody } from './transaction-body';
import type { PaymentSigningKeyPaths } from './payment-signing-key-path';
import type { Hex } from './hex';

export class Transaction {
  private constructor(readonly value: Hex) {}

  public static async create(
    privateKey: PrivateKey,
    paymentSigningKeyPaths: PaymentSigningKeyPaths,
    transactionBody: TransactionBody
  ) {
    const result = await RnCardanoWallet.transaction(
      privateKey.value,
      JSON.stringify(paymentSigningKeyPaths.toJSON()),
      JSON.stringify(transactionBody.toJSON())
    );

    return new Transaction(result);
  }
}
