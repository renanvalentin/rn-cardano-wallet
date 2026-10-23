#ifndef PRIVATEKEY_H
#define PRIVATEKEY_H

#include <string>
#include "rust.h"

class PrivateKeyData
{
public:
    PrivateKeyData(const char *mnemonic, const char *salt, const char *password);
    ~PrivateKeyData();
    char *getValue();

private:
    PrivateKey *raw;
};

#endif /* PRIVATEKEY_H */
