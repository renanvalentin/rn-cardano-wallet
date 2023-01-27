#include <iostream>
#include "private-key.h"

using namespace std;

PrivateKeyData::PrivateKeyData(const char *mnemonic, const char *salt, const char *password)
{
    raw = private_key_create(mnemonic, salt, password);
    if (raw == nullptr)
    {
        throw "Invalid Private Key";
    }
}

PrivateKeyData::~PrivateKeyData()
{
    cout << "c++:PrivateKeyData:destructor";
    private_key_free(raw);
}

char *PrivateKeyData::getValue()
{
    return raw->value;
}