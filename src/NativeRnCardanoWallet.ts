import type { TurboModule } from 'react-native';
import { TurboModuleRegistry } from 'react-native';

export interface Spec extends TurboModule {
  privateKey(entropy: string, password: string): Promise<string>;
  publicAccountKey(base64Bip32PrivateKey: string): Promise<string>;
  validateMnemonic(mnemonic: string): boolean;
  bech32Address(
    bech32PublicAccountKey: string,
    changeIndex: number,
    index: number
  ): Promise<string>;
  paymentAddress(
    network: number,
    bech32PaymentVerificationKey: string,
    bech32StakeVerificationKey: string
  ): Promise<string>;
}

export default TurboModuleRegistry.getEnforcing<Spec>('RnCardanoWallet');
