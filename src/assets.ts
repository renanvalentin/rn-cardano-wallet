import type { AssetsJSON } from '@emurgo/cardano-serialization-lib-nodejs';
import { AssetName } from './asset-name';

type AssetsData = Record<AssetName['value'], bigint>;

export class Assets {
  private constructor(readonly value: AssetsData) {}

  public static async create(value: AssetsData) {
    return new Assets(value);
  }

  insert(assetName: AssetName, value: bigint) {
    return new Assets({
      ...this.value,
      [assetName.value]: value,
    });
  }

  public static fromJSON(data: AssetsJSON) {
    const value = Object.entries(data).reduce<AssetsData>(
      (acc, [assetName, value]) => {
        const name = AssetName.create(assetName).value;

        return {
          ...acc,
          [name]: BigInt(value),
        };
      },
      {}
    );

    return new Assets(value);
  }

  serialize() {
    return Object.entries(this.value).reduce((acc, [assetName, value]) => {
      return {
        ...acc,
        [assetName]: value.toString(),
      };
    }, {});
  }
}
