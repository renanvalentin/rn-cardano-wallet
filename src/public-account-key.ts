import RnCardanoWallet from './NativeRnCardanoWallet';
import type { Bech32 } from './bech32';
import type { PrivateKey } from './private-key';

export class PublicAccountKey {
  private constructor(readonly value: Bech32) {}

  public static async create(privateKey: PrivateKey, password: string) {
    const publicKey = await RnCardanoWallet.publicAccountKey(
      privateKey.value,
      password
    );

    return new PublicAccountKey(publicKey);
  }
}
