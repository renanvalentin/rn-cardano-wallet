#include <iostream>
#include "transaction-body.h"

using namespace std;

TransactionBodyData::TransactionBodyData(const char *configJson, const char *inputsJson, const char *outputJson, const char *bech32ChangeAddress, u_int64_t ttl)
{
    raw = transaction_body_create(configJson, inputsJson, outputJson, bech32ChangeAddress, ttl);
    if (raw == nullptr)
    {
        throw "Invalid Params";
    }
}

TransactionBodyData::~TransactionBodyData()
{
    cout << "c++:TransactionBodyData:destructor";
    transaction_body_free(raw);
}

const char *TransactionBodyData::getValue()
{
    return raw->value;
}