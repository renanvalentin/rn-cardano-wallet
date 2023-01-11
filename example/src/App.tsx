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
} from 'rn-cardano-wallet';

enum Stages {
  GeneratingPrivateKey = 'Generating private key',
  PrivateKeyCreated = 'Private Key Created',
  GeneratingPublicKey = 'Generating public key',
  PublicKeyCreated = 'Public Key Created',
  GeneratingPaymentVerificationKey = 'Generating Payment Verification Key',
  PaymentVerificationKeyCreated = 'Payment Verification Key Created',
  GeneratingStakeVerificationKey = 'Generating Stake Verification Key',
  StakeVerificationKeyCreated = 'Stake Verification Key Created',
  GeneratingPaymentAddress = 'Generating Payment Address',
  PaymentAddressCreated = 'Payment Address Created',
  BuildingTransaction = 'Building Transaction',
  TransactionReady = 'Transaction Ready',
}

interface State {
  privateKey?: PrivateKey;
  publicAccountKey?: PublicAccountKey;
  paymentVerificationKey?: PaymentVerificationKey;
  stakeVerificationKey?: StakeVerificationKey;
  paymentAddress?: PaymentAddress;
  transactionBody?: TransactionBody;
  stage: Stages;
}

export default function App() {
  const [state, setState] = useState<State>({
    privateKey: undefined,
    publicAccountKey: undefined,
    stage: Stages.GeneratingPrivateKey,
  });

  useEffect(() => {
    const pause = (n = 0) => new Promise((resolve) => setTimeout(resolve, n));

    const op = async () => {
      await pause();

      const privateKey = await PrivateKey.create(
        'weapon shock brick category tragic grocery filter lecture cement wreck hundred rigid diagram brain country possible monitor urge among gasp love swarm picture risk',
        ''
      );

      setState((s) => ({
        ...s,
        privateKey,
        stage: Stages.PrivateKeyCreated,
      }));

      await pause();

      setState((s) => ({
        ...s,
        stage: Stages.GeneratingPublicKey,
      }));

      const publicAccountKey = await PublicAccountKey.create(privateKey);

      assert.equal(
        publicAccountKey.value,
        'xpub18xysay7qk28yp99x5g8g6j65h96jm8ptgkpucg0redax23yrntzaexwe7af7m566qlrdl40h99rwqnng9l0ry702qz79tmnpep277hcmv66ev',
        'Public account key doesnt match'
      );

      await pause();

      setState((s) => ({
        ...s,
        stage: Stages.PublicKeyCreated,
        publicAccountKey,
      }));

      await pause();

      setState((s) => ({
        ...s,
        stage: Stages.GeneratingPaymentVerificationKey,
      }));

      const paymentVerificationKey = await PaymentVerificationKey.create(
        publicAccountKey
      );

      assert.equal(
        paymentVerificationKey.value,
        'xpub1urgdmy77t2y04glr436dnfgxzz0e8zfp2jl0cgkh0gvrztljl2mpfmuz7t7575xfkn6n77l67kz25g39nncp4gqh44pkh3k9nnr075s2n92ht',
        'Payment verification key doesnt match'
      );

      setState((s) => ({
        ...s,
        stage: Stages.PaymentVerificationKeyCreated,
        paymentVerificationKey,
      }));

      await pause();

      setState((s) => ({
        ...s,
        stage: Stages.GeneratingPaymentVerificationKey,
      }));

      const stakeVerificationKey = await StakeVerificationKey.create(
        publicAccountKey
      );

      assert.equal(
        stakeVerificationKey.value,
        'xpub1vwklcehh2q3fhvvctkjkds9l0m59jsc95cpdq7qsrkenyn0zngyn8pk3kyu6a5pzczxz7e0agx88rnzlnatgwv83lyp2899fve6evksppm7wr',
        'Stake verification key doesnt match'
      );

      setState((s) => ({
        ...s,
        stage: Stages.StakeVerificationKeyCreated,
        stakeVerificationKey,
      }));

      await pause();

      setState((s) => ({
        ...s,
        stage: Stages.GeneratingPaymentAddress,
      }));

      const network = 0;

      const paymentAddress = await PaymentAddress.create(
        network,
        paymentVerificationKey,
        stakeVerificationKey
      );

      await pause();

      setState((s) => ({
        ...s,
        stage: Stages.PaymentAddressCreated,
        paymentAddress,
      }));

      await pause();

      setState((s) => ({
        ...s,
        stage: Stages.BuildingTransaction,
      }));

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

      const transactionUnspentOutput = TransactionUnspentOutput.create(
        TransactionInput.create(
          TransactionHash.create(
            '488afed67b342d41ec08561258e210352fba2ac030c98a8199bc22ec7a27ccf1'
          ),
          0
        ),
        TransactionOutput.create(
          Address.create(
            'addr_test1qra2njhhucffhtfwq3zyvz3h9huqd87d83zay44h2a6nj0lt8erv04n4weca43v4jhdrpqsc5f5mh2zx0pa4k04v34eq32w05z'
          ),
          Value.create(20_000_000n)
        )
      );

      const transactionUnspentOutputs = TransactionUnspentOutputs.create([
        transactionUnspentOutput,
      ]);

      const txOuput = TransactionOutput.create(
        Address.create(
          'addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w'
        ),
        Value.create(8_000_000n)
      );

      const changeAddress = Address.create(
        'addr_test1gz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerspqgpsqe70et'
      );

      const txBuild = await TransactionBuilder.build({
        config: txConfig,
        inputs: transactionUnspentOutputs,
        output: txOuput,
        changeAddress,
        ttl: 1000,
      });

      await pause();

      setState((s) => ({
        ...s,
        stage: Stages.TransactionReady,
        transactionBody: txBuild,
      }));
    };

    op();
  }, []);

  return (
    <SafeAreaView style={styles.container}>
      <View>
        <ScrollView>
          <Text>{state.stage}</Text>

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

          {/* <Text>Public Account Key: {publicAccountKey.value}</Text> */}
          {/* <Button onPress={() => setFirst(!first)} title="toggle" /> */}
          {/* 
          {first &&
            Array.from({ length: 1 }).map((_, i) => {
              const pk = PrivateKey.create(
                'bind hammer ethics slush company special bean alcohol witness stuff umbrella erase police jelly silent firm frog burger glimpse survey tribe fatigue glance icon',
                ''
              );
              return <Text key={i + 1}>Key 2: {pk.value}</Text>;
            })}*/}
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
