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

struct Bech32Address {
  char *value;
};

struct PaymentAddress {
  char *value;
};

extern "C" {

PrivateKey *private_key_create(const char *c_entropy, const char *c_password);

void private_key_free(PrivateKey *private_key_ptr);

PublicAccountKey *public_account_key_create(const char *c_bip32_private_key);

void public_account_key_free(PublicAccountKey *public_account_key_ptr);

bool mnemonic_is_valid(const char *c_mnemonic);

Bech32Address *bech32_address_create(const char *c_bech32_public_key,
                                     uint32_t change_index,
                                     uint32_t index);

void bech32_address_free(Bech32Address *bech32_address_ptr);

PaymentAddress *payment_address_create(uint8_t network,
                                       const char *c_bip32_payment_verification_key,
                                       const char *c_bip32_stake_verification_key);

void payment_address_free(PaymentAddress *payment_address_ptr);

} // extern "C"
