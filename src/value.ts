import type { ValueJSON } from '@emurgo/cardano-serialization-lib-nodejs';
import type { Coin } from './coin';
import { MultiAsset } from './multi-asset';

export class Value {
  private constructor(readonly coin: Coin, readonly multiasset?: MultiAsset) {}

  public static create(coin: Coin, multiasset?: MultiAsset) {
    return new Value(coin, multiasset);
  }

  public static fromJSON(data: ValueJSON) {
    return new Value(
      BigInt(data.coin),
      data.multiasset ? MultiAsset.fromJSON(data.multiasset) : undefined
    );
  }
}
