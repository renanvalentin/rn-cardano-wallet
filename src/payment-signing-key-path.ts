interface CreateArgs {
  accountIndex?: number;
  changeIndex: number;
  index: number;
}

export class PaymentSigningKeyPath {
  private constructor(
    readonly accountIndex: number,
    readonly changeIndex: number,
    readonly index: number
  ) {}

  public static create({ accountIndex = 0, changeIndex, index }: CreateArgs) {
    return new PaymentSigningKeyPath(accountIndex, changeIndex, index);
  }
}

export class PaymentSigningKeyPaths {
  private constructor(readonly value: PaymentSigningKeyPath[]) {}

  public static create(value: PaymentSigningKeyPath[]) {
    return new PaymentSigningKeyPaths(value);
  }

  public toJSON() {
    return this.value.map((path) => ({
      account_index: path.accountIndex,
      change_index: path.changeIndex,
      index: path.index,
    }));
  }
}
