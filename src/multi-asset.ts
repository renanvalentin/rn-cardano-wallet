import { ScriptHash } from './script-hash';
import { Assets } from './assets';
import type { MultiAssetJSON } from '@emurgo/cardano-serialization-lib-nodejs';

type MultiAssetData = Record<ScriptHash['value'], Assets>;

export class MultiAsset {
  private constructor(readonly value: MultiAssetData) {}

  public static create(value: MultiAssetData) {
    return new MultiAsset(value);
  }

  insert(policyId: ScriptHash, assets: Assets) {
    return new MultiAsset({
      ...this.value,
      [policyId.value]: assets,
    });
  }

  public static fromJSON(data: MultiAssetJSON) {
    const value = Object.entries(data).reduce<MultiAssetData>(
      (acc, [scriptHash, value]) => {
        const name = ScriptHash.create(scriptHash).value;

        return {
          ...acc,
          [name]: Assets.fromJSON(value),
        };
      },
      {}
    );

    return new MultiAsset(value);
  }

  serialize() {
    return Object.entries(this.value).reduce((acc, [policyId, assets]) => {
      return {
        ...acc,
        [policyId]: assets.serialize(),
      };
    }, {});
  }
}
