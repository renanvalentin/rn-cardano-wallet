import RnCardanoWallet from './NativeRnCardanoWallet';
import type { Bech32 } from './bech32';
import { ValidationError } from './errors';
import { Mnemonic } from './mnemonic';

export class PrivateKey {
  private constructor(readonly value: Bech32) {}

  public static async create(mnemonic: string, password: string) {
    if (!(await Mnemonic.validate(mnemonic))) {
      throw new ValidationError('Invalid mnemonic');
    }

    const privateKey = await RnCardanoWallet.privateKey(mnemonic, password);

    return new PrivateKey(privateKey);
  }
}
