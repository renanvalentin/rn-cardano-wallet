#ifndef PUBLICACCOUNTKEY_H
#define PUBLICACCOUNTKEY_H

#include <string>
#include "rust.h"

class PublicAccountKeyData
{
public:
    PublicAccountKeyData(const std::string);
    ~PublicAccountKeyData();
    std::string getValue();

private:
    PublicAccountKey *raw;
};

#endif /* PUBLICACCOUNTKEY_H */
