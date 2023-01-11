import type { UnitIntervalJSON } from '@emurgo/cardano-serialization-lib-nodejs';

export class UnitInterval {
  private constructor(
    readonly denominator: string,
    readonly numerator: string
  ) {}

  public static create(denominator: string, numerator: string) {
    return new UnitInterval(denominator, numerator);
  }

  public static fromJSON(data: UnitIntervalJSON) {
    return new UnitInterval(data.denominator, data.numerator);
  }
}
