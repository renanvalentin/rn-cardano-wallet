import type { Hex } from './hex';

export class AssetId {
  private constructor(readonly value: Hex) {}

  public static create(value: Hex) {
    return new AssetId(value);
  }
}
