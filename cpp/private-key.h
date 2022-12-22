#ifndef PRIVATEKEY_H
#define PRIVATEKEY_H

#include <string>
#include "rust.h"

class PrivateKeyData
{
public:
    PrivateKeyData(const std::string, const std::string);
    ~PrivateKeyData();
    std::string getValue();

private:
    PrivateKey *raw;
};

#endif /* PRIVATEKEY_H */
