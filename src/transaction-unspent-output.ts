import type { TransactionInput } from './transaction-input';
import type { TransactionOutput } from './transaction-output';

export class TransactionUnspentOutput {
  private constructor(
    readonly input: TransactionInput,
    readonly output: TransactionOutput
  ) {}

  public static create(input: TransactionInput, output: TransactionOutput) {
    return new TransactionUnspentOutput(input, output);
  }

  public serialize() {
    return {
      input: this.input.serialize(),
      output: this.output.serialize(),
    };
  }
}
