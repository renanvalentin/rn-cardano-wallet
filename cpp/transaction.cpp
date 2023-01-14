#include <iostream>
#include "transaction.h"

using namespace std;

TransactionData::TransactionData(const uint8_t *privateKeyBytes, size_t privateKeyBytesLen, const char *c_payment_signing_key_paths_json, const char *c_transaction_body_json)
{
    raw = transaction_create(privateKeyBytes, privateKeyBytesLen, c_payment_signing_key_paths_json, c_transaction_body_json);
    if (raw == nullptr)
    {
        throw "Invalid Args";
    }
}

TransactionData::~TransactionData()
{
    cout << "c++:TransactionData:destructor";
    transaction_free(raw);
}

const char *TransactionData::getValue()
{
    return raw->value;
}