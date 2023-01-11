import type { TransactionUnspentOutput } from './transaction-unspent-output';

export class TransactionUnspentOutputs {
  private constructor(readonly value: TransactionUnspentOutput[]) {}

  public static create(value: TransactionUnspentOutput[]) {
    return new TransactionUnspentOutputs(value);
  }

  public add(transactionUnspentOutput: TransactionUnspentOutput) {
    return new TransactionUnspentOutputs([
      ...this.value,
      transactionUnspentOutput,
    ]);
  }

  public serialize() {
    return this.value.map((v) => v.serialize());
  }
}
