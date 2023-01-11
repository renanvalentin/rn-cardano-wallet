import type {
  TransactionInputJSON,
  TransactionInputsJSON,
} from '@emurgo/cardano-serialization-lib-nodejs';
import { TransactionHash } from './transaction-hash';

export class TransactionInput {
  private constructor(
    readonly transactionHash: TransactionHash,
    readonly index: number
  ) {}

  public static create(transactionHash: TransactionHash, index: number) {
    return new TransactionInput(transactionHash, index);
  }

  public static fromJSON(data: TransactionInputJSON) {
    return new TransactionInput(
      TransactionHash.create(data.transaction_id),
      data.index
    );
  }

  public serialize() {
    return {
      transaction_id: this.transactionHash.value,
      index: this.index,
    };
  }
}

export class TransactionInputs {
  private constructor(readonly value: TransactionInput[]) {}

  public static create(value: TransactionInput[]) {
    return new TransactionInputs(value);
  }

  public static fromJSON(data: TransactionInputsJSON) {
    return new TransactionInputs(data.map((d) => TransactionInput.fromJSON(d)));
  }
}
