#import "RnCardanoWallet.h"
#import "KeyAgent.h"
#import "UUID.h"

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
    
    auto pk = rncardanowallet::privateKey([entropy UTF8String], [password UTF8String]);
    
    NSString *privateKeyFromCString = [NSString stringWithCString:pk.c_str() encoding:[NSString defaultCStringEncoding]];
    
    NSData *plainText = [privateKeyFromCString dataUsingEncoding:NSUTF8StringEncoding];
    
    NSData *cipherText = [keyAgent encrypt:plainText];
    
    NSString *result = [cipherText base64EncodedStringWithOptions:0];
    
    NSLog(@"encrypted string %@",result);
    
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
    
    NSData *clearBip32PrivateKey = [KeyAgent decrypt:tag cipherText:data withError:&error];
    
    if(error){
        NSString *errorCode = [@(error.code) stringValue];
        NSString *errorMessage = [error localizedDescription];
        return reject(errorCode, errorMessage, nil);
    }
    
    NSString *bip32PrivateKey = [[NSString alloc] initWithData:clearBip32PrivateKey encoding:NSUTF8StringEncoding];
    
    auto publicAccountKey = rncardanowallet::publicAccountKey([bip32PrivateKey UTF8String]);
    
    NSString *result = [NSString stringWithCString:publicAccountKey.c_str() encoding:[NSString defaultCStringEncoding]];
    
    resolve(result);
}

- (void)bech32Address:(NSString *)bech32PublicAccountKey changeIndex:(double)changeIndex index:(double)index resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject
{
    auto bech32Address = rncardanowallet::bech32Address([bech32PublicAccountKey UTF8String], changeIndex, index);
    
    NSString *result = [NSString stringWithCString:bech32Address.c_str() encoding:[NSString defaultCStringEncoding]];
    
    resolve(result);
}

- (void)paymentAddress:(double)network
bech32PaymentVerificationKey:(NSString *)bech32PaymentVerificationKey
bech32StakeVerificationKey:(NSString *)bech32StakeVerificationKey
               resolve:(RCTPromiseResolveBlock)resolve
                reject:(RCTPromiseRejectBlock)reject
{
    auto paymentAddress = rncardanowallet::paymentAddress(network, [bech32PaymentVerificationKey UTF8String], [bech32StakeVerificationKey UTF8String]);
    
    NSString *result = [NSString stringWithCString:paymentAddress.c_str() encoding:[NSString defaultCStringEncoding]];
    
    resolve(result);
}

- (NSNumber *)validateMnemonic:(NSString *)mnemonic
{
    auto isValid = rncardanowallet::validateMnemonic([mnemonic UTF8String]);
    
    NSNumber *result = [NSNumber numberWithBool:(isValid)];
    
    return result;
}

- (void)transactionBody:(NSString *)configJson
             inputsJson:(NSString *)inputsJson
             outputJson:(NSString *)outputJson
    bech32ChangeAddress:(NSString *)bech32ChangeAddress
                    ttl:(double)ttl
                resolve:(RCTPromiseResolveBlock)resolve
                 reject:(RCTPromiseRejectBlock)reject
{
    auto transactionBody = rncardanowallet::transactionBody([configJson UTF8String], [inputsJson UTF8String], [outputJson UTF8String], [bech32ChangeAddress UTF8String], ttl);
    
    NSString *result = [NSString stringWithCString:transactionBody.c_str() encoding:[NSString defaultCStringEncoding]];
    
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
