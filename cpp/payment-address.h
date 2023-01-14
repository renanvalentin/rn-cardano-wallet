#ifndef PAYMENTADDRESS_H
#define PAYMENTADDRESS_H

#include <string>
#include "rust.h"

class PaymentAddressData
{
public:
    PaymentAddressData(uint8_t network, const char *bech32PaymentVerificationKey, const char *bech32StakeVerificationKey);
    ~PaymentAddressData();
    const char *getValue();

private:
    PaymentAddress *raw;
};

#endif /* PAYMENTADDRESS_H */
