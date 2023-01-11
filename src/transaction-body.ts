import type { TransactionBodyJSON } from '@emurgo/cardano-serialization-lib-nodejs';
import { TransactionInputs } from './transaction-input';
import { TransactionOutputs } from './transaction-output';

export class TransactionBody {
  private constructor(
    readonly fee: string,
    readonly inputs: TransactionInputs,
    readonly outputs: TransactionOutputs,
    readonly ttl?: string,
    readonly json?: TransactionBodyJSON
  ) {}

  public static fromJSON(data: TransactionBodyJSON) {
    return new TransactionBody(
      data.fee,
      TransactionInputs.fromJSON(data.inputs),
      TransactionOutputs.fromJSON(data.outputs),
      data.ttl ? data.ttl : undefined,
      data
    );
  }

  public toJSON() {
    return this.json;
  }
}
