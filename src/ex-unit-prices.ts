import type { ExUnitPricesJSON } from '@emurgo/cardano-serialization-lib-nodejs';
import type { UnitInterval } from './unit-interval';

export class ExUnitPrices {
  private constructor(
    readonly memPrice: UnitInterval,
    readonly stepPrice: UnitInterval
  ) {}

  public static create(memPrice: UnitInterval, stepPrice: UnitInterval) {
    return new ExUnitPrices(memPrice, stepPrice);
  }

  public static fromJSON(data: ExUnitPricesJSON) {
    return new ExUnitPrices(data.mem_price, data.step_price);
  }
}
