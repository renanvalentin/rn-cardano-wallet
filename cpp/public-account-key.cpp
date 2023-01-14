#include <iostream>
#include "public-account-key.h"

using namespace std;

PublicAccountKeyData::PublicAccountKeyData(const uint8_t *bytes, size_t len)
{
    raw = public_account_key_create(bytes, len);
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