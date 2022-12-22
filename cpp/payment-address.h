#ifndef PAYMENTADDRESS_H
#define PAYMENTADDRESS_H

#include <string>
#include "rust.h"

class PaymentAddressData
{
public:
    PaymentAddressData(uint8_t network, const std::string bech32PaymentVerificationKey, const std::string bech32StakeVerificationKey);
    ~PaymentAddressData();
    std::string getValue();

private:
    PaymentAddress *raw;
};

#endif /* PAYMENTADDRESS_H */
