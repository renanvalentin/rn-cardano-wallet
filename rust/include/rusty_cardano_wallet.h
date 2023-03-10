#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct PrivateKey {
  char *value;
};

struct PublicAccountKey {
  char *value;
};

struct MnemonicValidation {
  bool value;
};

struct Bech32Address {
  char *value;
};

struct PaymentAddress {
  char *value;
};

struct TransactionBody {
  char *value;
};

struct Transaction {
  char *value;
};

extern "C" {

PrivateKey *private_key_create(const char *c_entropy, const char *c_password);

void private_key_free(PrivateKey *private_key_ptr);

PublicAccountKey *public_account_key_create(const uint8_t *c_bip32_private_key_bytes,
                                            size_t c_bip32_private_key_len);

void public_account_key_free(PublicAccountKey *public_account_key_ptr);

MnemonicValidation *mnemonic_validation_create(const char *c_mnemonic);

void mnemonic_validation_free(MnemonicValidation *mnemonic_validation_ptr);

Bech32Address *bech32_address_create(const char *c_bech32_public_key,
                                     uint32_t change_index,
                                     uint32_t index);

void bech32_address_free(Bech32Address *bech32_address_ptr);

PaymentAddress *payment_address_create(uint8_t network,
                                       const char *c_bip32_payment_verification_key,
                                       const char *c_bip32_stake_verification_key);

void payment_address_free(PaymentAddress *payment_address_ptr);

TransactionBody *transaction_body_create(const char *c_config_json,
                                         const char *c_inputs_json,
                                         const char *c_output_json,
                                         const char *c_bech32_change_address,
                                         uint64_t ttl);

void transaction_body_free(TransactionBody *transaction_body_ptr);

Transaction *transaction_create(const uint8_t *c_bip32_private_key_bytes,
                                size_t c_bip32_private_key_len,
                                const char *c_payment_signing_key_paths_json,
                                const char *c_transaction_body_json);

void transaction_free(Transaction *transaction_ptr);

} // extern "C"
