import React, { useEffect, useState } from 'react';

import { StyleSheet, View, Text, ScrollView, SafeAreaView } from 'react-native';

import assert from 'assert';
import {
  PaymentVerificationKey,
  PrivateKey,
  PublicAccountKey,
  StakeVerificationKey,
  TransactionBody,
  TransactionBuilder,
  TransactionHash,
  TransactionInput,
  TransactionOutput,
  TransactionBuilderConfig,
  TransactionUnspentOutput,
  Address,
  Value,
  TransactionUnspentOutputs,
  PaymentAddress,
  Transaction,
  PaymentSigningKeyPath,
  PaymentSigningKeyPaths,
} from 'rn-cardano-wallet';

interface State {
  privateKey?: PrivateKey;
  publicAccountKey?: PublicAccountKey;
  paymentVerificationKey?: PaymentVerificationKey;
  stakeVerificationKey?: StakeVerificationKey;
  paymentAddress?: PaymentAddress;
  transactionBody?: TransactionBody;
  transaction?: Transaction;
}

export default function App() {
  const [state, setState] = useState<State>({
    privateKey: undefined,
    publicAccountKey: undefined,
    paymentVerificationKey: undefined,
    stakeVerificationKey: undefined,
    paymentAddress: undefined,
    transactionBody: undefined,
  });

  useEffect(() => {
    const op = async () => {
      const privateKey = await PrivateKey.create(
        'weapon shock brick category tragic grocery filter lecture cement wreck hundred rigid diagram brain country possible monitor urge among gasp love swarm picture risk',
        ''
      );

      const publicAccountKey = await PublicAccountKey.create(privateKey);

      assert.equal(
        publicAccountKey.value,
        'xpub18xysay7qk28yp99x5g8g6j65h96jm8ptgkpucg0redax23yrntzaexwe7af7m566qlrdl40h99rwqnng9l0ry702qz79tmnpep277hcmv66ev',
        'Public account key doesnt match'
      );

      const paymentVerificationKey = await PaymentVerificationKey.create(
        publicAccountKey
      );

      assert.equal(
        paymentVerificationKey.value,
        'xpub1urgdmy77t2y04glr436dnfgxzz0e8zfp2jl0cgkh0gvrztljl2mpfmuz7t7575xfkn6n77l67kz25g39nncp4gqh44pkh3k9nnr075s2n92ht',
        'Payment verification key doesnt match'
      );

      const stakeVerificationKey = await StakeVerificationKey.create(
        publicAccountKey
      );

      assert.equal(
        stakeVerificationKey.value,
        'xpub1vwklcehh2q3fhvvctkjkds9l0m59jsc95cpdq7qsrkenyn0zngyn8pk3kyu6a5pzczxz7e0agx88rnzlnatgwv83lyp2899fve6evksppm7wr',
        'Stake verification key doesnt match'
      );

      const network = 0;

      const paymentAddress = await PaymentAddress.create(
        network,
        paymentVerificationKey,
        stakeVerificationKey
      );

      assert.equal(
        paymentAddress.value,
        'addr_test1qpmulz4p20fp0dezmh5s4k9duudu45upun53c287w9s8f78trfmehefs0j9jnhhlkn9t6ctsjq4guvtf8hs9kmtqqa8qzfct4l',
        'Payment address doesnt match'
      );

      const txConfig = TransactionBuilderConfig.create({
        feeAlgo: {
          coefficient: 44,
          constant: 155381,
        },
        coinsPerUtxoByte: 34482,
        poolDeposit: 500000000,
        keyDeposit: 2000000,
        maxValueSize: 4000,
        maxTxSize: 8000,
        preferPureChange: true,
      });

      const transactionUnspentOutput1 = TransactionUnspentOutput.create(
        TransactionInput.create(
          TransactionHash.create(
            '2edf46a289c160372ab9b2ad4673bf20ae5590583d94a58734d88b118b433584'
          ),
          3
        ),
        TransactionOutput.create(
          Address.create(
            'addr_test1qqp74hmqkw4gtdmadc3rr49gj639l06jf0ldg5kqexm2348trfmehefs0j9jnhhlkn9t6ctsjq4guvtf8hs9kmtqqa8qeh269e'
          ),
          Value.create(100_000_000n)
        )
      );

      const transactionUnspentOutput2 = TransactionUnspentOutput.create(
        TransactionInput.create(
          TransactionHash.create(
            '03e1623048b7a9cd97319308dbb95151c5f47d4c129ed067b13a247630b4b04e'
          ),
          4
        ),
        TransactionOutput.create(
          Address.create(
            'addr_test1qq8vvzpg6l0j2te4kpvl40n8jpdyv0ps65m7wv0wgknckghtrfmehefs0j9jnhhlkn9t6ctsjq4guvtf8hs9kmtqqa8qvtznfy'
          ),
          Value.create(36_250_000n)
        )
      );

      const transactionUnspentOutputs = TransactionUnspentOutputs.create([
        transactionUnspentOutput1,
        transactionUnspentOutput2,
      ]);

      const txOuput = TransactionOutput.create(
        Address.create(
          'addr_test1qpjj6ayphjkcxh3fygz90emlmg6gq8n73cf2zn80zh768j8trfmehefs0j9jnhhlkn9t6ctsjq4guvtf8hs9kmtqqa8qfa7pyn'
        ),
        Value.create(120_000_000n)
      );

      const changeAddress = Address.create(
        'addr_test1vz2uwaq7n3wjj66autet46n2je3w99amwcgsyjvp9ah5twcmwxqe6'
      );

      const transactionBody = await TransactionBuilder.build({
        config: txConfig,
        inputs: transactionUnspentOutputs,
        output: txOuput,
        changeAddress,
        ttl: 18044450 + 3600 * 6,
      });

      const paymentSigningKeyPath1 = PaymentSigningKeyPath.create({
        changeIndex: 0,
        index: 3,
      });

      const paymentSigningKeyPath2 = PaymentSigningKeyPath.create({
        changeIndex: 0,
        index: 2,
      });

      const paymentSigningKeyPaths = PaymentSigningKeyPaths.create([
        paymentSigningKeyPath1,
        paymentSigningKeyPath2,
      ]);

      const transaction = await Transaction.create(
        privateKey,
        paymentSigningKeyPaths,
        transactionBody
      );

      setState({
        privateKey,
        publicAccountKey,
        paymentVerificationKey,
        stakeVerificationKey,
        paymentAddress,
        transaction,
        transactionBody,
      });
    };

    op();
  }, []);

  return (
    <SafeAreaView style={styles.container}>
      <View>
        <ScrollView>
          {state.privateKey && (
            <Text style={styles.text}>
              Private Key: {state.privateKey.value}
            </Text>
          )}

          {state.publicAccountKey && (
            <Text style={styles.text}>
              Public account key: {state.publicAccountKey.value}
            </Text>
          )}

          {state.paymentVerificationKey && (
            <Text style={styles.text}>
              Payment verification key: {state.paymentVerificationKey.value}
            </Text>
          )}

          {state.stakeVerificationKey && (
            <Text style={styles.text}>
              Stake verification key: {state.stakeVerificationKey.value}
            </Text>
          )}

          {state.paymentAddress && (
            <Text style={styles.text}>
              Payment Address (m/1852'/1815'/0'/0/0):{' '}
              {state.paymentAddress.value}
            </Text>
          )}

          {state.transactionBody && (
            <Text style={styles.text}>
              Transaction Body: {JSON.stringify(state.transactionBody.toJSON())}
            </Text>
          )}

          {state.transaction && (
            <Text style={styles.text}>
              Signed Transaction: {JSON.stringify(state.transaction.value)}
            </Text>
          )}
        </ScrollView>
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  box: {
    width: 60,
    height: 60,
    marginVertical: 20,
  },
  text: {
    marginTop: 20,
    color: '#FF0000',
  },
});
