#include <iostream>
#include "public-account-key.h"

using namespace std;

PublicAccountKeyData::PublicAccountKeyData(const uint8_t *privateKeyBytes, size_t privateKeyBytesLen, const char *password)
{
    raw = public_account_key_create(privateKeyBytes, privateKeyBytesLen, password);
    if (raw == nullptr)
    {
        throw "Invalid Private Key";
    }
}

PublicAccountKeyData::~PublicAccountKeyData()
{
    cout << "c++:PublicAccountKeyData:destructor";
    public_account_key_free(raw);
}

const char *PublicAccountKeyData::getValue()
{
    return raw->value;
}