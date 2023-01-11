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

## IOS

### Private Key & Secure Enclave

This lib follows the same strategy used by VivoPay. See [article](https://medium.com/coinmonks/how-we-created-an-insanely-secure-crypto-wallet-617917063a06#:~:text=The%20Secure%20Enclave%20is%20a,cannot%20leave%20the%20Secure%20Enclave.)

> This is how VivoPay uses the Secure Enclave. When a user launches
> VivoPay for the first time, the following things happen in this order:
>
> 1.  VivoPay will create a wallet with a Harmony One private key in memory (this is done without use of the Secure Enclave).
> 2.  The Secure Enclave then creates a unique private key to encrypt the wallet.
> 3.  The encrypted wallet is saved on disk.
> 4.  The wallet is deleted from memory.
>
> When a user creates a new transaction that needs to be signed, the
> following things happen in this order:
>
> 1.  The encrypted wallet is loaded from disk and decrypted by the Secure Enclave.
> 2.  The transaction is signed by the wallet.
> 3.  The wallet is deleted from memory.

## Contributing

See the [contributing guide](CONTRIBUTING.md) to learn how to contribute to the repository and the development workflow.

## License

MIT

---

Made with [create-react-native-library](https://github.com/callstack/react-native-builder-bob)
