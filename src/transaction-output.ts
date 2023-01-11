import type {
  TransactionOutputJSON,
  TransactionOutputsJSON,
} from '@emurgo/cardano-serialization-lib-nodejs';
import { Address } from './address';
import { Value } from './value';

// TODO: missing plutus_data & script_ref

export class TransactionOutput {
  private constructor(readonly address: Address, readonly amount: Value) {}

  public static create(address: Address, amount: Value) {
    return new TransactionOutput(address, amount);
  }

  public static fromJSON(data: TransactionOutputJSON) {
    return new TransactionOutput(
      Address.create(data.address),
      Value.fromJSON(data.amount)
    );
  }

  public serialize() {
    return {
      address: this.address.value,
      amount: {
        coin: this.amount.coin.toString(),
        multiasset: this.amount.multiasset?.serialize() ?? null,
      },
      plutus_data: null,
      script_ref: null,
    };
  }
}

export class TransactionOutputs {
  private constructor(readonly value: TransactionOutput[]) {}

  public static create(value: TransactionOutput[]) {
    return new TransactionOutputs(value);
  }

  public static fromJSON(data: TransactionOutputsJSON) {
    return new TransactionOutputs(
      data.map((d) => TransactionOutput.fromJSON(d))
    );
  }
}
