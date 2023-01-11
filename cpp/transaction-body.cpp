#include <iostream>
#include "transaction-body.h"

using namespace std;

TransactionBodyData::TransactionBodyData(const std::string configJson, const std::string inputsJson, const std::string outputJson, const std::string bech32ChangeAddress, u_int64_t ttl)
{
    raw = transaction_body_create(configJson.c_str(), inputsJson.c_str(), outputJson.c_str(), bech32ChangeAddress.c_str(), ttl);
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

std::string TransactionBodyData::getValue()
{
    return raw->value;
}