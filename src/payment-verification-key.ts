import RnCardanoWallet from './NativeRnCardanoWallet';
import type { Bech32 } from './bech32';
import type { PublicAccountKey } from './public-account-key';

export class PaymentVerificationKey {
  private constructor(readonly value: Bech32) {}

  public static async create(
    publicAccountKey: PublicAccountKey,
    index: number = 0
  ) {
    const paymentVerificationKey = await RnCardanoWallet.bech32Address(
      publicAccountKey.value,
      0,
      index
    );

    return new PaymentVerificationKey(paymentVerificationKey);
  }
}
