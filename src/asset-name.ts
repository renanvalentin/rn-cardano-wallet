import type { Hex } from './hex';

export class AssetName {
  private constructor(readonly value: Hex) {}

  public static create(value: Hex) {
    return new AssetName(value);
  }
}
