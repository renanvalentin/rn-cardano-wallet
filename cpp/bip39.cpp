#include <iostream>
#include "bip39.h"

using namespace std;

namespace bip39
{
    bool validateMnemonic(const std::string mnemonic)
    {
        return mnemonic_is_valid(mnemonic.c_str());
    }
}