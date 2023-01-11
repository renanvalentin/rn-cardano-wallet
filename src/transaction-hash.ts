export class TransactionHash {
  private constructor(readonly value: string) {}

  public static create(value: string) {
    return new TransactionHash(value);
  }
}
