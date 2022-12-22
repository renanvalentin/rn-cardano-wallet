#include <iostream>
#include "payment-address.h"

using namespace std;

PaymentAddressData::PaymentAddressData(uint8_t network, const std::string bech32PaymentVerificationKey, const std::string bech32StakeVerificationKey)
{
    raw = payment_address_create(network, bech32PaymentVerificationKey.c_str(), bech32StakeVerificationKey.c_str());
    if (raw == nullptr)
    {
        throw "Invalid Params";
    }
}

PaymentAddressData::~PaymentAddressData()
{
    cout << "c++:PaymentAddressData:destructor";
    payment_address_free(raw);
}

std::string PaymentAddressData::getValue()
{
    return raw->value;
}