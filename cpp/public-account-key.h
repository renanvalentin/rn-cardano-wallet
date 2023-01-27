#ifndef PUBLICACCOUNTKEY_H
#define PUBLICACCOUNTKEY_H

#include <string>
#include "rust.h"

class PublicAccountKeyData
{
public:
    PublicAccountKeyData(const uint8_t *privateKeyBytes, size_t privateKeyBytesLen, const char *password);
    ~PublicAccountKeyData();
    const char *getValue();

private:
    PublicAccountKey *raw;
};

#endif /* PUBLICACCOUNTKEY_H */
