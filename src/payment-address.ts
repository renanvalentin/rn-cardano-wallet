import RnCardanoWallet from './NativeRnCardanoWallet';
import type { Bech32 } from './bech32';
import type { PaymentVerificationKey } from './payment-verification-key';
import type { StakeVerificationKey } from './stake-verification-key';

export class PaymentAddress {
  private constructor(readonly value: Bech32) {}

  public static async create(
    network: number,
    paymentVerificationKey: PaymentVerificationKey,
    stakeVerificationKey: StakeVerificationKey
  ) {
    const paymentAddress = await RnCardanoWallet.paymentAddress(
      network,
      paymentVerificationKey.value,
      stakeVerificationKey.value
    );

    return new PaymentAddress(paymentAddress);
  }
}
