#include <iostream>
#include "transaction.h"

using namespace std;

TransactionData::TransactionData(const uint8_t *privateKeyBytes, size_t privateKeyBytesLen, const char *password, const char *cPaymentSigningKeyPathsJson, const char *cTransactionBodyJson)
{
    raw = transaction_create(privateKeyBytes, privateKeyBytesLen, password, cPaymentSigningKeyPathsJson, cTransactionBodyJson);
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