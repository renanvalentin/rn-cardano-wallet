#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint32_t ITER = 19162;

constexpr static const uintptr_t SALT_SIZE = 32;

constexpr static const uintptr_t NONCE_SIZE = 12;

constexpr static const uintptr_t KEY_SIZE = 32;

constexpr static const uintptr_t TAG_SIZE = 16;

constexpr static const uintptr_t METADATA_SIZE = ((SALT_SIZE + NONCE_SIZE) + TAG_SIZE);

constexpr static const uintptr_t SALT_START = 0;

constexpr static const uintptr_t SALT_END = (SALT_START + SALT_SIZE);

constexpr static const uintptr_t NONCE_START = SALT_END;

constexpr static const uintptr_t NONCE_END = (NONCE_START + NONCE_SIZE);

constexpr static const uintptr_t TAG_START = NONCE_END;

constexpr static const uintptr_t TAG_END = (TAG_START + TAG_SIZE);

constexpr static const uintptr_t ENCRYPTED_START = TAG_END;

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

PrivateKey *private_key_create(const char *c_mnemonic, const char *c_salt, const char *c_password);

void private_key_free(PrivateKey *private_key_ptr);

PublicAccountKey *public_account_key_create(const uint8_t *c_bip32_private_key_bytes,
                                            size_t c_bip32_private_key_len,
                                            const char *c_password);

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
                                const char *c_password,
                                const char *c_payment_signing_key_paths_json,
                                const char *c_transaction_body_json);

void transaction_free(Transaction *transaction_ptr);

} // extern "C"
