#ifndef RNCARDANOWALLET_H
#define RNCARDANOWALLET_H

#include <string>

namespace rncardanowallet
{
  std::string privateKey(const std::string entropy, const std::string password);
  std::string publicAccountKey(const std::string bip32PrivateKey);
  std::string bech32Address(const std::string bech32PublicAccountKey, uint32_t changeIndex, uint32_t index);
  std::string paymentAddress(uint8_t network, const std::string bech32PaymentVerificationKey, const std::string bech32StakeVerificationKey);
  bool validateMnemonic(std::string mnemonic);
  std::string transactionBody(const std::string configJson, const std::string inputsJson, const std::string outputJson, const std::string bech32ChangeAddress, u_int64_t ttl);
}

#endif /* RNCARDANOWALLET_H */
