#ifndef TRANSACTIONBODY_H
#define TRANSACTIONBODY_H

#include <string>
#include "rust.h"

class TransactionBodyData
{
public:
    TransactionBodyData(const char *configJson, const char *inputsJson, const char *outputJson, const char *bech32ChangeAddress, u_int64_t ttl);
    ~TransactionBodyData();
    const char *getValue();

private:
    TransactionBody *raw;
};

#endif /* TRANSACTIONBODY_H */
