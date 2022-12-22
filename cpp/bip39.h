#ifndef BIP39_H
#define BIP39_H

#include <string>
#include "rust.h"

namespace bip39
{
    bool validateMnemonic(std::string mnemonic);
}

#endif /* BIP39_H */
