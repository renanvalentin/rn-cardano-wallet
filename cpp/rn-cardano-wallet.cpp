#include "rn-cardano-wallet.h"

#ifndef LIST_H_
#define LIST_H_

#include "rust.h"
#include "private-key.h"
#include "public-account-key.h"
#include "bip39.h"
#include "bech32-address.h"
#include "payment-address.h"
#include "transaction-body.h"

#endif

namespace rncardanowallet
{
    std::string privateKey(const std::string entropy, const std::string password)
    {
        PrivateKeyData privateKey(entropy.c_str(), password.c_str());

        return privateKey.getValue();
    }

    std::string publicAccountKey(const std::string bip32PrivateKey)
    {
        PublicAccountKeyData publicAccountKey(bip32PrivateKey.c_str());

        return publicAccountKey.getValue();
    }

    std::string bech32Address(const std::string bech32PublicAccountKey, u_int32_t changeIndex, uint32_t index)
    {
        Bech32AddressData bech32Address(bech32PublicAccountKey.c_str(), changeIndex, index);

        return bech32Address.getValue();
    }

    std::string paymentAddress(uint8_t network, const std::string bech32PaymentVerificationKey, const std::string bech32StakeVerificationKey)
    {
        PaymentAddressData paymentAddress(network, bech32PaymentVerificationKey.c_str(), bech32StakeVerificationKey.c_str());

        return paymentAddress.getValue();
    }

    bool validateMnemonic(const std::string mnemonic)
    {
        return bip39::validateMnemonic(mnemonic);
    }

    std::string transactionBody(const std::string configJson, const std::string inputsJson, const std::string outputJson, std::string bech32ChangeAddress, u_int64_t ttl)
    {
        TransactionBodyData transactionBody(configJson.c_str(), inputsJson.c_str(), outputJson.c_str(), bech32ChangeAddress.c_str(), ttl);

        return transactionBody.getValue();
    }
}
