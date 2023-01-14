#include <iostream>
#include "mnemonic-validation.h"

using namespace std;

MnemonicValidationData::MnemonicValidationData(const char *mnemonic)
{
    raw = mnemonic_validation_create(mnemonic);
    if (raw == nullptr)
    {
        throw "Invalid Mnemonic";
    }
}

MnemonicValidationData::~MnemonicValidationData()
{
    cout << "c++:MnemonicValidationData:destructor";
    mnemonic_validation_free(raw);
}

bool MnemonicValidationData::getValue()
{
    return raw->value;
}