#include <iostream>
#include "private-key.h"

using namespace std;

PrivateKeyData::PrivateKeyData(const std::string entropy, const std::string password)
{
    raw = private_key_create(entropy.c_str(), password.c_str());
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

std::string PrivateKeyData::getValue()
{
    return raw->value;
}