#ifndef RNCARDANOWALLET_H
#define RNCARDANOWALLET_H

#include <string>

namespace rncardanowallet
{
  std::string privateKey(std::string entropy, std::string password);
  std::string publicAccountKey(std::string bip32PrivateKey);
  std::string bech32Address(std::string bech32PublicAccountKey, uint32_t changeIndex, uint32_t index);
  std::string paymentAddress(uint8_t network, std::string bech32PaymentVerificationKey, std::string bech32StakeVerificationKey);
  bool validateMnemonic(std::string mnemonic);
}

#endif /* RNCARDANOWALLET_H */
