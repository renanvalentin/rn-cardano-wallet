import type { ProtocolParamUpdateJSON } from '@emurgo/cardano-serialization-lib-nodejs';
import { ExUnits } from './ex-unit';
import { ExUnitPrices } from './ex-unit-prices';
import { Nonce } from './nonce';
import { ProtocolVersion } from './protocol-version';
import { UnitInterval } from './unit-interval';

type CostModel = string[];

interface Costmdls {
  [k: string]: CostModel;
}

export class ProtocolParamUpdate {
  private constructor(
    readonly adaPerUtxoByte?: string | null,
    readonly collateralPercentage?: number | null,
    readonly costModels?: Costmdls | null,
    readonly d?: UnitInterval | null,
    readonly executionCosts?: ExUnitPrices | null,
    readonly expansionRate?: UnitInterval | null,
    readonly extraEntropy?: Nonce | null,
    readonly keyDeposit?: string | null,
    readonly maxBlockBodySize?: number | null,
    readonly maxBlockExUnits?: ExUnits | null,
    readonly maxBlockHeaderSize?: number | null,
    readonly maxCollateralInputs?: number | null,
    readonly maxEpoch?: number | null,
    readonly maxTxExUnits?: ExUnits | null,
    readonly maxTxSize?: number | null,
    readonly maxValueSize?: number | null,
    readonly minPoolCost?: string | null,
    readonly minfeeA?: string | null,
    readonly minfeeB?: string | null,
    readonly nOpt?: number | null,
    readonly poolDeposit?: string | null,
    readonly poolPledgeInfluence?: UnitInterval | null,
    readonly protocolVersion?: ProtocolVersion | null,
    readonly treasuryGrowthRate?: UnitInterval | null
  ) {}

  public static fromJSON(data: ProtocolParamUpdateJSON) {
    return new ProtocolParamUpdate(
      data.ada_per_utxo_byte,
      data.collateral_percentage,
      data.cost_models,
      data.d ? UnitInterval.fromJSON(data.d) : null,
      data.execution_costs ? ExUnitPrices.fromJSON(data.execution_costs) : null,
      data.expansion_rate ? UnitInterval.fromJSON(data.expansion_rate) : null,
      data.extra_entropy?.hash ? Nonce.create(data.extra_entropy.hash) : null,
      data.key_deposit,
      data.max_block_body_size,
      data.max_block_ex_units
        ? ExUnits.fromJSON(data.max_block_ex_units)
        : null,
      data.max_block_header_size,
      data.max_collateral_inputs,
      data.max_epoch,
      data.max_tx_ex_units ? ExUnits.fromJSON(data.max_tx_ex_units) : null,
      data.max_tx_size,
      data.max_value_size,
      data.min_pool_cost,
      data.minfee_a,
      data.minfee_b,
      data.n_opt,
      data.pool_deposit,
      data.pool_pledge_influence,
      data.protocol_version
        ? ProtocolVersion.fromJSON(data.protocol_version)
        : null,
      data.treasury_growth_rate
        ? UnitInterval.fromJSON(data.treasury_growth_rate)
        : null
    );
  }
}

type ProposedProtocolParameterUpdates = {
  [k: string]: ProtocolParamUpdate;
};

export class Update {
  private constructor(
    readonly epoch?: number,
    readonly proposedProtocolParameterUpdates?: ProposedProtocolParameterUpdates
  ) {}

  public static create(
    epoch?: number,
    proposedProtocolParameterUpdates?: ProposedProtocolParameterUpdates
  ) {
    return new Update(epoch, proposedProtocolParameterUpdates);
  }
}
