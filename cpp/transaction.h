#ifndef TRANSACTION_H
#define TRANSACTION_H

#include <string>
#include "rust.h"

class TransactionData
{
public:
    TransactionData(const uint8_t *privateKeyBytes, size_t privateKeyBytesLen, const char *c_payment_signing_key_paths_json, const char *c_transaction_body_json);
    ~TransactionData();
    const char *getValue();

private:
    Transaction *raw;
};

#endif /* TRANSACTION_H */
