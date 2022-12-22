import React, { useEffect, useState } from 'react';

import {
  StyleSheet,
  View,
  Text,
  Button,
  ScrollView,
  SafeAreaView,
} from 'react-native';
import {
  PrivateKey,
  PublicAccountKey,
  PaymentVerificationKey,
  StakeVerificationKey,
  PaymentAddress,
} from 'rn-cardano-wallet';
import assert from 'assert';

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
}

interface State {
  privateKey?: PrivateKey;
  publicAccountKey?: PublicAccountKey;
  paymentVerificationKey?: PaymentVerificationKey;
  stakeVerificationKey?: StakeVerificationKey;
  paymentAddress?: PaymentAddress;
  stage: Stages;
}

export default function App() {
  const [state, setState] = useState<State>({
    privateKey: undefined,
    publicAccountKey: undefined,
    stage: Stages.GeneratingPrivateKey,
  });

  useEffect(() => {
    const pause = (n: number = 0) =>
      new Promise((resolve) => setTimeout(resolve, n));

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
    };

    op();
  }, []);

  return (
    <SafeAreaView style={styles.container}>
      <View>
        <Text>{state.stage}</Text>

        {state.privateKey && (
          <Text style={styles.text}>Private Key: {state.privateKey.value}</Text>
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
            Payment Address (m/1852'/1815'/0'/0/0): {state.paymentAddress.value}
          </Text>
        )}

        {/* <Text>Public Account Key: {publicAccountKey.value}</Text> */}
        {/* <Button onPress={() => setFirst(!first)} title="toggle" /> */}
        {/* <ScrollView>
          {first &&
            Array.from({ length: 1 }).map((_, i) => {
              const pk = PrivateKey.create(
                'bind hammer ethics slush company special bean alcohol witness stuff umbrella erase police jelly silent firm frog burger glimpse survey tribe fatigue glance icon',
                ''
              );
              return <Text key={i + 1}>Key 2: {pk.value}</Text>;
            })}
        </ScrollView> */}
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
