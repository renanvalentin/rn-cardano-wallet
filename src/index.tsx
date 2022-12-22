import { ValidationError } from './errors';
import RnCardanoWallet from './NativeRnCardanoWallet';

type Bech32 = string;

class Mnemonic {
  static validate(mnemonic: string) {
    return RnCardanoWallet.validateMnemonic(mnemonic);
  }
}

export class PrivateKey {
  private constructor(readonly value: Bech32) {}

  public static async create(mnemonic: string, password: string) {
    if (!Mnemonic.validate(mnemonic)) {
      throw new ValidationError('Invalid mnemonic');
    }

    const privateKey = await RnCardanoWallet.privateKey(mnemonic, password);

    return new PrivateKey(privateKey);
  }
}

export class PublicAccountKey {
  private constructor(readonly value: Bech32) {}

  public static async create(privateKey: PrivateKey) {
    const publicKey = await RnCardanoWallet.publicAccountKey(privateKey.value);

    return new PublicAccountKey(publicKey);
  }
}

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
