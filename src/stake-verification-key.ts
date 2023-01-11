import RnCardanoWallet from './NativeRnCardanoWallet';
import type { Bech32 } from './bech32';
import type { PublicAccountKey } from './public-account-key';

export class StakeVerificationKey {
  private constructor(readonly value: Bech32) {}

  public static async create(publicAccountKey: PublicAccountKey) {
    const stakeVerificationKey = await RnCardanoWallet.bech32Address(
      publicAccountKey.value,
      2,
      0
    );

    return new StakeVerificationKey(stakeVerificationKey);
  }
}
