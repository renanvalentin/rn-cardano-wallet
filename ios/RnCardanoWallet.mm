#import "RnCardanoWallet.h"
#import "KeyAgent.h"
#import "UUID.h"
#import "private-key.h"
#import "public-account-key.h"
#import "bech32-address.h"
#import "payment-address.h"
#import "mnemonic-validation.h"
#import "transaction-body.h"
#import "transaction.h"

@implementation RnCardanoWallet
RCT_EXPORT_MODULE()

- (void)privateKey:(NSString *)entropy password:(NSString *)password resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject
{
    NSString *uuid = [UUID generate];
    NSData *tag = [uuid dataUsingEncoding:NSUTF8StringEncoding];
    NSError *error;
    
    KeyAgent *keyAgent = [KeyAgent createKeyAgent:tag withError:&error];
    
    if(error) {
        NSString *errorCode = [@(error.code) stringValue];
        NSString *errorMessage = [error localizedDescription];
        return reject(errorCode, errorMessage, nil);
    }
        
    const char *entropyCString = [entropy UTF8String];
    const char *passwordCString = [password UTF8String];
    
    PrivateKeyData privateKey(entropyCString, passwordCString);
    const char *privateKeyCString =privateKey.getValue();

    NSString *privateKeyFromCString = [NSString stringWithCString:privateKeyCString encoding:[NSString defaultCStringEncoding]];
    
    NSData *plainText = [privateKeyFromCString dataUsingEncoding:NSUTF8StringEncoding];

    NSData *cipherText = [keyAgent encrypt:plainText];

    NSString *result = [cipherText base64EncodedStringWithOptions:0];

    resolve([NSString stringWithFormat:@"%@:%@", uuid, result]);
}

- (void)publicAccountKey:(NSString *)base64Bip32PrivateKey resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject
{
    NSArray  *payload = [base64Bip32PrivateKey componentsSeparatedByString: @":"];
    
    NSString *uuid = payload[0];
    NSData *tag = [uuid dataUsingEncoding:NSUTF8StringEncoding];
    
    NSString *cipherText = payload[1];
    NSData *data = [[NSData alloc] initWithBase64EncodedString:cipherText options:0];
    
    NSError *error;
    
    NSMutableData *clearBip32PrivateKey = [KeyAgent decrypt:tag cipherText:data withError:&error];
        
    if(error){
        NSString *errorCode = [@(error.code) stringValue];
        NSString *errorMessage = [error localizedDescription];
        return reject(errorCode, errorMessage, nil);
    }
    
    const uint8_t *clearBip32PrivateKeyBytes = (uint8_t *)[clearBip32PrivateKey bytes];
        
    PublicAccountKeyData publicAccountKey(clearBip32PrivateKeyBytes, clearBip32PrivateKey.length);
    const char *publicAccountKeyCString = publicAccountKey.getValue();

    [clearBip32PrivateKey resetBytesInRange:NSMakeRange(0, [clearBip32PrivateKey length])];

    NSString *result = [NSString stringWithCString:publicAccountKeyCString encoding:[NSString defaultCStringEncoding]];

    resolve(result);
}

- (void)bech32Address:(NSString *)bech32PublicAccountKey changeIndex:(double)changeIndex index:(double)index resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject
{
    const char *bech32PublicAccountKeyCString =[bech32PublicAccountKey UTF8String];
    
    Bech32AddressData bech32Address(bech32PublicAccountKeyCString, changeIndex, index);
    const char *bech32AddressCString = bech32Address.getValue();
    
    
    NSString *result = [NSString stringWithCString:bech32AddressCString encoding:[NSString defaultCStringEncoding]];
    
    resolve(result);
}

- (void)paymentAddress:(double)network
bech32PaymentVerificationKey:(NSString *)bech32PaymentVerificationKey
bech32StakeVerificationKey:(NSString *)bech32StakeVerificationKey
               resolve:(RCTPromiseResolveBlock)resolve
                reject:(RCTPromiseRejectBlock)reject
{
    const char *bech32PaymentVerificationKeyCString = [bech32PaymentVerificationKey UTF8String];
    const char *bech32StakeVerificationKeyCString = [bech32StakeVerificationKey UTF8String];
    
    PaymentAddressData paymentAddress(network, bech32PaymentVerificationKeyCString, bech32StakeVerificationKeyCString);
    const char *paymentAddressCString = paymentAddress.getValue();
    
    
    NSString *result = [NSString stringWithCString:paymentAddressCString encoding:[NSString defaultCStringEncoding]];
    
    resolve(result);
}

- (void)validateMnemonic:(NSString *)mnemonic resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject
{
    const char *mnemonicCString = [mnemonic UTF8String];
    
    MnemonicValidationData mnemonicValidation(mnemonicCString);
    BOOL isValid = mnemonicValidation.getValue();
    
    NSNumber *result = [NSNumber numberWithBool:isValid];
    
    resolve(result);
}

- (void)transactionBody:(NSString *)configJson
             inputsJson:(NSString *)inputsJson
             outputJson:(NSString *)outputJson
    bech32ChangeAddress:(NSString *)bech32ChangeAddress
                    ttl:(double)ttl
                resolve:(RCTPromiseResolveBlock)resolve
                 reject:(RCTPromiseRejectBlock)reject
{
    const char *configJsonCString = [configJson UTF8String];
    const char *inputsJsonCString = [inputsJson UTF8String];
    const char *outputJsonCString = [outputJson UTF8String];
    const char *bech32ChangeAddressCString = [bech32ChangeAddress UTF8String];
    
    TransactionBodyData transactionBody(configJsonCString, inputsJsonCString, outputJsonCString, bech32ChangeAddressCString, ttl);
    const char *transactionBodyCString = transactionBody.getValue();
    
    
    NSString *result = [NSString stringWithCString:transactionBodyCString encoding:[NSString defaultCStringEncoding]];
    
    resolve(result);
}

- (void)transaction:(NSString *)base64Bip32PrivateKey
paymentSigningKeyPathsJson:(NSString *)paymentSigningKeyPathsJson
transactionBodyJson:(NSString *)transactionBodyJson
            resolve:(RCTPromiseResolveBlock)resolve
             reject:(RCTPromiseRejectBlock)reject
{
    const char *paymentSigningKeyPathsJsonCString = [paymentSigningKeyPathsJson UTF8String];
    const char *transactionBodyJsonCString = [transactionBodyJson UTF8String];
    
    NSArray *payload = [base64Bip32PrivateKey componentsSeparatedByString: @":"];
    
    NSString *uuid = payload[0];
    NSData *tag = [uuid dataUsingEncoding:NSUTF8StringEncoding];
    
    NSString *cipherText = payload[1];
    NSData *data = [[NSData alloc] initWithBase64EncodedString:cipherText options:0];
    
    NSError *error;
    
    NSMutableData *clearBip32PrivateKey = [KeyAgent decrypt:tag cipherText:data withError:&error];
    
    if(error){
        NSString *errorCode = [@(error.code) stringValue];
        NSString *errorMessage = [error localizedDescription];
        return reject(errorCode, errorMessage, nil);
    }
    
    const uint8_t *clearBip32PrivateKeyBytes = (uint8_t *)[clearBip32PrivateKey bytes];
    
    TransactionData transaction(clearBip32PrivateKeyBytes, clearBip32PrivateKey.length, paymentSigningKeyPathsJsonCString, transactionBodyJsonCString);
    const char *transactionCString = transaction.getValue();
    
    [clearBip32PrivateKey resetBytesInRange:NSMakeRange(0, [clearBip32PrivateKey length])];
    
    NSString *result = [NSString stringWithCString:transactionCString encoding:[NSString defaultCStringEncoding]];
    
    resolve(result);
}

// Don't compile this code when we build for the old architecture.
#ifdef RCT_NEW_ARCH_ENABLED
- (std::shared_ptr<facebook::react::TurboModule>)getTurboModule:
(const facebook::react::ObjCTurboModule::InitParams &)params
{
    return std::make_shared<facebook::react::NativeRnCardanoWalletSpecJSI>(params);
}
#endif

@end
