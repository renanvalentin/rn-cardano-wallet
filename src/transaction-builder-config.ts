interface FeeAlgo {
  coefficient: number;
  constant: number;
}
interface TransactionBuilderConfigArgs {
  readonly feeAlgo?: FeeAlgo;
  readonly coinsPerUtxoByte?: number;
  readonly poolDeposit?: number;
  readonly keyDeposit?: number;
  readonly maxValueSize?: number;
  readonly maxTxSize?: number;
  readonly preferPureChange?: boolean;
}

export class TransactionBuilderConfig {
  private constructor(
    readonly feeAlgo?: FeeAlgo,
    readonly coinsPerUtxoByte?: number,
    readonly poolDeposit?: number,
    readonly keyDeposit?: number,
    readonly maxValueSize?: number,
    readonly maxTxSize?: number,
    readonly preferPureChange?: boolean
  ) {}

  public static create({
    feeAlgo,
    coinsPerUtxoByte,
    poolDeposit,
    keyDeposit,
    maxValueSize,
    maxTxSize,
    preferPureChange,
  }: TransactionBuilderConfigArgs) {
    return new TransactionBuilderConfig(
      feeAlgo,
      coinsPerUtxoByte,
      poolDeposit,
      keyDeposit,
      maxValueSize,
      maxTxSize,
      preferPureChange
    );
  }

  public serialize() {
    return {
      fee_algo: this.feeAlgo,
      coins_per_utxo_byte: this.coinsPerUtxoByte,
      pool_deposit: this.poolDeposit,
      key_deposit: this.keyDeposit,
      max_value_size: this.maxValueSize,
      max_tx_size: this.maxTxSize,
      prefer_pure_change: this.preferPureChange,
    };
  }
}
