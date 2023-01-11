import type { Bech32 } from './bech32';

export class Address {
  private constructor(readonly value: Bech32) {}

  public static create(value: Bech32) {
    return new Address(value);
  }
}
