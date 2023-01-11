import RnCardanoWallet from './NativeRnCardanoWallet';

export class Mnemonic {
  static validate(mnemonic: string) {
    return RnCardanoWallet.validateMnemonic(mnemonic);
  }
}
