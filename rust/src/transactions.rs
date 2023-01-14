use cardano_serialization_lib::{
    address::Address,
    crypto::Bip32PrivateKey,
    fees::LinearFee,
    tx_builder::{TransactionBuilder, TransactionBuilderConfig, TransactionBuilderConfigBuilder},
    utils::{to_bignum, TransactionUnspentOutputs},
    Transaction, TransactionBody, TransactionOutput,
};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

pub fn create_transaction_body(
    config_json: &str,
    inputs_json: &str,
    output_json: &str,
    bech32_change_address: &str,
    ttl: u64,
) -> Result<TransactionBody, String> {
    let transaction_unspent_outputs = match inputs_json_to_transaction_unspent_outputs(inputs_json)
    {
        Ok(t) => t,
        Err(err) => return Err(err),
    };

    let transaction_output = match output_json_to_transaction_output(output_json) {
        Ok(t) => t,
        Err(err) => return Err(err),
    };

    let config = match config_json_to_config(config_json) {
        Ok(c) => c,
        Err(err) => return Err(err),
    };

    let transaction_builder_config = match create_transaction_builder_config(&config) {
        Ok(c) => c,
        Err(err) => return Err(err),
    };

    let change_address = match Address::from_bech32(bech32_change_address) {
        Ok(addr) => addr,
        Err(err) => return Err(err.to_string()),
    };

    let mut tx_builder = TransactionBuilder::new(&transaction_builder_config);

    match tx_builder.add_output(&transaction_output) {
        Err(err) => return Err(err.to_string()),
        _ => (),
    };

    match tx_builder.add_inputs_from(
        &transaction_unspent_outputs,
        cardano_serialization_lib::tx_builder::CoinSelectionStrategyCIP2::RandomImproveMultiAsset,
    ) {
        Err(err) => return Err(err.to_string()),
        _ => (),
    }

    tx_builder.set_ttl_bignum(&to_bignum(ttl));

    let change_added = match tx_builder.add_change_if_needed(&change_address) {
        Ok(needed) => needed,
        Err(err) => return Err(err.to_string()),
    };

    println!("change needed {}", change_added);

    let transaction_body = match tx_builder.build() {
        Ok(t) => t,
        Err(err) => return Err(err.to_string()),
    };

    Ok(transaction_body)
}

// pub fn create_transaction(
//     bip32_private_key: &Bip32PrivateKey,
//     payment_signing_key_paths_json: &str,
//     transaction_body_json: &str,
// ) -> Result<TransactionBody, String> {
//     let transaction_unspent_outputs = match inputs_json_to_transaction_unspent_outputs(inputs_json)
//     {
//         Ok(t) => t,
//         Err(err) => return Err(err),
//     };

//     let transaction_output = match output_json_to_transaction_output(output_json) {
//         Ok(t) => t,
//         Err(err) => return Err(err),
//     };

//     let config = match config_json_to_config(config_json) {
//         Ok(c) => c,
//         Err(err) => return Err(err),
//     };

//     let transaction_builder_config = match create_transaction_builder_config(&config) {
//         Ok(c) => c,
//         Err(err) => return Err(err),
//     };

//     let change_address = match Address::from_bech32(bech32_change_address) {
//         Ok(addr) => addr,
//         Err(err) => return Err(err.to_string()),
//     };

//     let mut tx_builder = TransactionBuilder::new(&transaction_builder_config);

//     match tx_builder.add_output(&transaction_output) {
//         Err(err) => return Err(err.to_string()),
//         _ => (),
//     };

//     match tx_builder.add_inputs_from(
//         &transaction_unspent_outputs,
//         cardano_serialization_lib::tx_builder::CoinSelectionStrategyCIP2::RandomImproveMultiAsset,
//     ) {
//         Err(err) => return Err(err.to_string()),
//         _ => (),
//     }

//     tx_builder.set_ttl_bignum(&to_bignum(ttl));

//     let change_added = match tx_builder.add_change_if_needed(&change_address) {
//         Ok(needed) => needed,
//         Err(err) => return Err(err.to_string()),
//     };

//     println!("change needed {}", change_added);

//     let transaction_body = match tx_builder.build() {
//         Ok(t) => t,
//         Err(err) => return Err(err.to_string()),
//     };

//     Ok(transaction_body)
// }

#[derive(Serialize, Deserialize)]
struct FeeAlgo {
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    coefficient: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    constant: u64,
}

#[derive(Serialize, Deserialize)]
struct Config {
    fee_algo: Option<FeeAlgo>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    coins_per_utxo_byte: Option<u64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pool_deposit: Option<u64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    key_deposit: Option<u64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    max_value_size: Option<u32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    max_tx_size: Option<u32>,
    prefer_pure_change: Option<bool>,
}

fn config_json_to_config(config_json: &str) -> Result<Config, String> {
    match serde_json::from_str(config_json) {
        Ok(c) => Ok(c),
        Err(err) => Err(err.to_string()),
    }
}

fn create_transaction_builder_config(config: &Config) -> Result<TransactionBuilderConfig, String> {
    let mut tx_builder_cfg = TransactionBuilderConfigBuilder::new();

    if let Some(fee_algo) = &config.fee_algo {
        tx_builder_cfg = tx_builder_cfg.fee_algo(&LinearFee::new(
            &to_bignum(fee_algo.coefficient),
            &to_bignum(fee_algo.constant),
        ));
    }

    if let Some(coins_per_utxo_byte) = config.coins_per_utxo_byte {
        tx_builder_cfg = tx_builder_cfg.coins_per_utxo_byte(&to_bignum(coins_per_utxo_byte));
    }

    if let Some(pool_deposit) = config.pool_deposit {
        tx_builder_cfg = tx_builder_cfg.pool_deposit(&to_bignum(pool_deposit));
    }

    if let Some(key_deposit) = config.key_deposit {
        tx_builder_cfg = tx_builder_cfg.key_deposit(&to_bignum(key_deposit));
    }

    if let Some(max_value_size) = config.max_value_size {
        tx_builder_cfg = tx_builder_cfg.max_value_size(max_value_size);
    }

    if let Some(max_tx_size) = config.max_tx_size {
        tx_builder_cfg = tx_builder_cfg.max_tx_size(max_tx_size);
    }

    if let Some(prefer_pure_change) = config.prefer_pure_change {
        tx_builder_cfg = tx_builder_cfg.prefer_pure_change(prefer_pure_change);
    }

    match tx_builder_cfg.build() {
        Ok(cfg) => Ok(cfg),
        Err(err) => Err(err.to_string()),
    }
}

fn inputs_json_to_transaction_unspent_outputs(
    inputs_json: &str,
) -> Result<TransactionUnspentOutputs, String> {
    match TransactionUnspentOutputs::from_json(inputs_json) {
        Ok(tx) => Ok(tx),
        Err(err) => Err(err.to_string()),
    }
}

fn output_json_to_transaction_output(output_json: &str) -> Result<TransactionOutput, String> {
    match TransactionOutput::from_json(output_json) {
        Ok(tx) => Ok(tx),
        Err(err) => Err(err.to_string()),
    }
}
#[derive(Serialize, Deserialize)]
struct PaymentSigningKeyPath {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    change_index: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    index: u32,
}

fn payment_signing_key_paths_json_to_data(
    c_payment_signing_key_paths_json: &str,
) -> Result<Vec<PaymentSigningKeyPath>, String> {
    match serde_json::from_str(c_payment_signing_key_paths_json) {
        Ok(c) => Ok(c),
        Err(err) => Err(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use cardano_serialization_lib::{crypto::ScriptHash, AssetName};

    use super::*;

    #[test]
    fn test_payment_signing_key_paths_json_to_data() {
        let str_json = r#"
        [{
            "change_index": 0,
            "index": 0
         }]"#;

        let paths = payment_signing_key_paths_json_to_data(str_json).unwrap();

        assert_eq!(paths[0].change_index, 0);
        assert_eq!(paths[0].index, 0);
    }

    #[test]
    fn test_inputs_json_to_transaction_unspent_outputs() {
        let inputs_json = r#"
        [{
            "input": {
              "transaction_id":
                "6d8ab0f38c5748e6fd59e04ec162c098784b27cbaaca6d2d1ab702e01f29a97c",
              "index": 1
            },
            "output": {
              "address":
                "addr_test1qra2njhhucffhtfwq3zyvz3h9huqd87d83zay44h2a6nj0lt8erv04n4weca43v4jhdrpqsc5f5mh2zx0pa4k04v34eq32w05z",
              "amount": {
                "coin": "11538668",
                "multiasset": {
                  "789ef8ae89617f34c07f7f6a12e4d65146f958c0bc15a97b4ff169f1": {
                    "F8AB28C2": "2"
                  }
                }
              },
              "plutus_data": null,
              "script_ref": null
            }
          }]"#;

        let result = inputs_json_to_transaction_unspent_outputs(inputs_json).unwrap();

        let input = result.get(0).input();
        let output = result.get(0).output();

        assert_eq!(input.index(), 1);
        assert_eq!(
            input.transaction_id().to_string(),
            "6d8ab0f38c5748e6fd59e04ec162c098784b27cbaaca6d2d1ab702e01f29a97c"
        );
        assert_eq!(
            output.address().to_bech32(None).unwrap(),
            "addr_test1qra2njhhucffhtfwq3zyvz3h9huqd87d83zay44h2a6nj0lt8erv04n4weca43v4jhdrpqsc5f5mh2zx0pa4k04v34eq32w05z"
        );
        assert_eq!(output.amount().coin(), to_bignum(11538668));
        assert_eq!(
            output
                .amount()
                .multiasset()
                .unwrap()
                .get(
                    &ScriptHash::from_hex(
                        "789ef8ae89617f34c07f7f6a12e4d65146f958c0bc15a97b4ff169f1"
                    )
                    .unwrap()
                )
                .unwrap()
                .get(&AssetName::new(vec![248, 171, 40, 194,]).unwrap())
                .unwrap(),
            to_bignum(2)
        );
        assert_eq!(output.script_ref(), None);
        assert_eq!(output.plutus_data(), None);
    }

    #[test]
    fn test_output_json_to_transaction_output() {
        let output_json = r#"
        {
            "address":
                "addr_test1qra2njhhucffhtfwq3zyvz3h9huqd87d83zay44h2a6nj0lt8erv04n4weca43v4jhdrpqsc5f5mh2zx0pa4k04v34eq32w05z",
            "amount": {
                "coin": "11538668"
            }
         }"#;

        let output = output_json_to_transaction_output(output_json).unwrap();

        assert_eq!(
            output.address().to_bech32(None).unwrap(),
            "addr_test1qra2njhhucffhtfwq3zyvz3h9huqd87d83zay44h2a6nj0lt8erv04n4weca43v4jhdrpqsc5f5mh2zx0pa4k04v34eq32w05z"
        );
        assert_eq!(output.amount().coin(), to_bignum(11538668));
    }

    #[test]
    fn test_config_json_to_config() {
        let data = r#"
        {
            "fee_algo": {
                "coefficient": "44",
                "constant": "155381"
            },
            "coins_per_utxo_byte": "34482",
            "pool_deposit": "500000000",
            "key_deposit": "2000000",
            "max_value_size": "4000",
            "max_tx_size": "8000",
            "prefer_pure_change": false
        }"#;

        let config = config_json_to_config(data).unwrap();

        let fee_algo = config.fee_algo.unwrap();

        assert_eq!(fee_algo.coefficient, 44);
        assert_eq!(fee_algo.constant, 155381);
        assert_eq!(config.coins_per_utxo_byte.unwrap(), 34482);
        assert_eq!(config.pool_deposit.unwrap(), 500000000);
        assert_eq!(config.key_deposit.unwrap(), 2000000);
        assert_eq!(config.max_value_size.unwrap(), 4000);
        assert_eq!(config.max_tx_size.unwrap(), 8000);
        assert_eq!(config.prefer_pure_change.unwrap(), false);
    }
}
