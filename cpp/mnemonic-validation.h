#ifndef MNEMONICVALIDATION_H
#define MNEMONICVALIDATION_H

#include <string>
#include "rust.h"

class MnemonicValidationData
{
public:
    MnemonicValidationData(const char *mnemonic);
    ~MnemonicValidationData();
    bool getValue();

private:
    MnemonicValidation *raw;
};

#endif /* MNEMONICVALIDATION_H */
