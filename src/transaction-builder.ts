import RnCardanoWallet from './NativeRnCardanoWallet';
import type { Address } from './address';
import type { TransactionOutput } from './transaction-output';
import type { TransactionUnspentOutputs } from './transaction-unspent-outputs';
import type { TransactionBuilderConfig } from './transaction-builder-config';
import { TransactionBody } from './transaction-body';

interface BuildArgs {
  config: TransactionBuilderConfig;
  inputs: TransactionUnspentOutputs;
  output: TransactionOutput;
  changeAddress: Address;
  ttl: number;
}

export class TransactionBuilder {
  public static async build({
    config,
    inputs,
    output,
    changeAddress,
    ttl,
  }: BuildArgs) {
    const transactionBody = await RnCardanoWallet.transactionBody(
      JSON.stringify(config.serialize()),
      JSON.stringify(inputs.serialize()),
      JSON.stringify(output.serialize()),
      changeAddress.value,
      ttl
    );

    return TransactionBody.fromJSON(JSON.parse(transactionBody));
  }
}
