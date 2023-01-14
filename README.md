# rn-cardano-wallet

react native cardano wallet

## Installation

```sh
npm install rn-cardano-wallet
```

## Usage

### Key Generation

```js
import {
  PrivateKey,
  PublicAccountKey,
  PaymentVerificationKey,
  StakeVerificationKey,
  PaymentAddress,
} from 'rn-cardano-wallet';

// ...

const mnemonic = '';

const privateKey = await PrivateKey.create(mnemonic, '');

const publicAccountKey = await PublicAccountKey.create(privateKey);

const paymentVerificationKey = await PaymentVerificationKey.create(
  publicAccountKey
);

const stakeVerificationKey = await StakeVerificationKey.create(
  publicAccountKey
);

const paymentAddress = await PaymentAddress.create(
  network,
  paymentVerificationKey,
  stakeVerificationKey
);

paymentAddress.value; // addr_test1qpmulz4p20fp0dezmh5s4k9duudu45upun53c287w9s8f78trfmehefs0j9jnhhlkn9t6ctsjq4guvtf8hs9kmtqqa8qzfct4l
```

### Transactions

```js
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

const transactionBody = await TransactionBuilder.build({
  config: txConfig,
  inputs: transactionUnspentOutputs,
  output: txOuput,
  changeAddress,
  ttl: 1000,
});

transactionBody.toJSON();
/**
{
  "inputs": [
    {
      "transaction_id": "488afed67b342d41ec08561258e210352fba2ac030c98a8199bc22ec7a27ccf1",
      "index": 0
    }
  ],
  "outputs": [
    {
      "address": "addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w",
      "amount": {
        "coin": "8000000",
        "multiasset": null
      },
      "plutus_data": null,
      "script_ref": null
    },
    ...
  ],
  ...
}
**/
```

### Transaction Signing

Use `PaymentSigningKeyPath` for each unique `TransactionUnspentOutput` addresses.

```js
const paymentSigningKeyPath = PaymentSigningKeyPath.create({
  accountIndex: 0,
  changeIndex: 0,
  index: 3,
});

const paymentSigningKeyPaths = PaymentSigningKeyPaths.create([
  paymentSigningKeyPath,
]);

const transaction = await Transaction.create(
  privateKey,
  paymentSigningKeyPaths,
  transactionBody
);
**/
```

## IOS

### Private Key & Secure Enclave

The private key is encrypted and stored using the [Secure Enclave](https://developer.apple.com/documentation/security/certificate_key_and_trust_services/keys/protecting_keys_with_the_secure_enclave?language=objc) and loaded into memory using `NSMutableData` so the buffer can be zero out after usage.

It will only be required for:

- Creating new public key accounts (m/1852'/1815'/0');
- Signing transactions;

## Contributing

See the [contributing guide](CONTRIBUTING.md) to learn how to contribute to the repository and the development workflow.

## License

MIT

---

Made with [create-react-native-library](https://github.com/callstack/react-native-builder-bob)
