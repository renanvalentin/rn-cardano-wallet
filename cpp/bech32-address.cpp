#include <iostream>
#include "bech32-address.h"

using namespace std;

Bech32AddressData::Bech32AddressData(const std::string bech32PublicAccountKey, uint32_t changeIndex, uint32_t index)
{
    raw = bech32_address_create(bech32PublicAccountKey.c_str(), changeIndex, index);
    if (raw == nullptr)
    {
        throw "Invalid Params";
    }
}

Bech32AddressData::~Bech32AddressData()
{
    cout << "c++:Bech32AddressData:destructor";
    bech32_address_free(raw);
}

std::string Bech32AddressData::getValue()
{
    return raw->value;
}