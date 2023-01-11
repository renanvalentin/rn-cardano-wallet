import RnCardanoWallet from './NativeRnCardanoWallet';
import type { Bech32 } from './bech32';
import type { PrivateKey } from './private-key';

export class PublicAccountKey {
  private constructor(readonly value: Bech32) {}

  public static async create(privateKey: PrivateKey) {
    const publicKey = await RnCardanoWallet.publicAccountKey(privateKey.value);

    return new PublicAccountKey(publicKey);
  }
}
