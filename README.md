# rn-cardano-wallet

react native cardano wallet

## Installation

```sh
npm install rn-cardano-wallet
```

## Usage

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
