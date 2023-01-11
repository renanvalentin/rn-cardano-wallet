#ifndef TRANSACTIONBODY_H
#define TRANSACTIONBODY_H

#include <string>
#include "rust.h"

class TransactionBodyData
{
public:
    TransactionBodyData(const std::string configJson, const std::string inputsJson, const std::string outputJson, const std::string bech32ChangeAddress, u_int64_t ttl);
    ~TransactionBodyData();
    std::string getValue();

private:
    TransactionBody *raw;
};

#endif /* TRANSACTIONBODY_H */
