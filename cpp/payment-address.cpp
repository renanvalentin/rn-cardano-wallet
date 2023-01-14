#include <iostream>
#include "payment-address.h"

using namespace std;

PaymentAddressData::PaymentAddressData(uint8_t network, const char *bech32PaymentVerificationKey, const char *bech32StakeVerificationKey)
{
    raw = payment_address_create(network, bech32PaymentVerificationKey, bech32StakeVerificationKey);
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

const char *PaymentAddressData::getValue()
{
    return raw->value;
}