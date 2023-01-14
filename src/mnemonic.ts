import RnCardanoWallet from './NativeRnCardanoWallet';

export class Mnemonic {
  static async validate(mnemonic: string) {
    let result = await RnCardanoWallet.validateMnemonic(mnemonic);
    console.log(result);
    return result;
  }
}
