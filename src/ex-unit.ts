import type { ExUnitsJSON } from '@emurgo/cardano-serialization-lib-nodejs';

export class ExUnits {
  private constructor(readonly mem: string, readonly steps: string) {}

  public static create(mem: string, steps: string) {
    return new ExUnits(mem, steps);
  }

  public static fromJSON(data: ExUnitsJSON) {
    return new ExUnits(data.mem, data.steps);
  }
}
