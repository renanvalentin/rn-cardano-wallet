interface MintAssets {
  [k: string]: string;
}

export class Mint {
  private constructor(readonly value?: MintAssets) {}

  public static create(value: MintAssets) {
    return new Mint(value);
  }
}
