#include <iostream>
#include "public-account-key.h"

using namespace std;

PublicAccountKeyData::PublicAccountKeyData(const std::string bip32PrivateKey)
{
    raw = public_account_key_create(bip32PrivateKey.c_str());
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

std::string PublicAccountKeyData::getValue()
{
    return raw->value;
}