#ifndef BECH32ADDRESS_H
#define BECH32ADDRESS_H

#include <string>
#include "rust.h"

class Bech32AddressData
{
public:
    Bech32AddressData(const char *bech32PublicAccountKey, u_int32_t changeIndex, uint32_t index);
    ~Bech32AddressData();
    const char *getValue();

private:
    Bech32Address *raw;
};

#endif /* BECH32ADDRESS_H */
