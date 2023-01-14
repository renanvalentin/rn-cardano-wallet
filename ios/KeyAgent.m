//
//  KeyGen.m
//  RnCardanoWallet
//
//  Created by eliot on 05/01/23.
//  Copyright © 2023 Facebook. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "KeyAgent.h"

@implementation KeyAgent

+ (KeyAgent *)createKeyAgent:(NSData *)tag withError:(NSError **)errorPtr
{
    SecKeyRef publicKey = [KeyAgent findPublicKey:tag withError:errorPtr];
    
    if (publicKey != nil) {
        KeyAgent *keyAgent = [[KeyAgent alloc]initWithPublicKey:publicKey];
        return keyAgent;
    }
    
    publicKey = [KeyAgent createPublicKey:tag withError:errorPtr];
    
    KeyAgent *keyAgent = [[KeyAgent alloc]initWithPublicKey:publicKey];
    return keyAgent;
}

+ (SecKeyRef)createPublicKey:(NSData *)tag withError:(NSError **)errorPtr
{
#if TARGET_OS_SIMULATOR
    SecAccessControlRef access = SecAccessControlCreateWithFlags(
                                                                 kCFAllocatorDefault,
                                                                 kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
                                                                 kSecAccessControlPrivateKeyUsage,
                                                                 NULL);
#else
    SecAccessControlRef access = SecAccessControlCreateWithFlags(
                                                                 kCFAllocatorDefault,
                                                                 kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
                                                                 kSecAccessControlPrivateKeyUsage | kSecAccessControlBiometryAny,
                                                                 NULL);
#endif
    
    
    
    NSDictionary* attributes =
    @{ (id)kSecAttrKeyType: (id)kSecAttrKeyTypeECSECPrimeRandom,
       (id)kSecAttrKeySizeInBits: @256,
       (id)kSecAttrTokenID: (id)kSecAttrTokenIDSecureEnclave,
       (id)kSecPrivateKeyAttrs:
           @{ (id)kSecAttrIsPermanent: @YES,
              (id)kSecAttrApplicationTag:  tag,
              (id)kSecAttrAccessControl: (__bridge id)access,
           },
    };
    
    CFErrorRef error = NULL;
    SecKeyRef privateKey = SecKeyCreateRandomKey((__bridge CFDictionaryRef)attributes,
                                                 &error);
    if (!privateKey) {
        *errorPtr = CFBridgingRelease(error); // ARC takes ownership.
        return nil;
    }
    
    SecKeyRef publicKey = SecKeyCopyPublicKey(privateKey);
    
    if (privateKey) { CFRelease(privateKey); }
    if (access)     { CFRelease(access);     }
    
    return publicKey;
}

+ (SecKeyRef) findPublicKey:(NSData *)tag withError:(NSError **)errorPtr
{
    NSDictionary *getPublicKeyQuery = [KeyAgent createPublicKeyQuery:tag];
    
    SecKeyRef publicKey = NULL;
    OSStatus status = SecItemCopyMatching((__bridge CFDictionaryRef)getPublicKeyQuery,
                                          (CFTypeRef *)&publicKey);
    if (status!=errSecSuccess && status!=errSecItemNotFound) {
        *errorPtr = [KeyAgent createError:status];
        
        return nil;
    }
    
    if (!publicKey) {
        return nil;
    }
    
    return publicKey;
}

+ (NSDictionary *)createPrivateKeyQuery:(NSData *)tag
{
    NSDictionary *getquery = @{ (id)kSecClass: (id)kSecClassKey,
                                (id)kSecAttrApplicationTag: tag,
                                (id)kSecAttrKeyClass: (id)kSecAttrKeyClassPrivate,
                                (id)kSecReturnRef: @YES,
    };
    
    return getquery;
}

+ (NSDictionary *)createPublicKeyQuery:(NSData *)tag
{
    NSDictionary *getquery = @{ (id)kSecClass: (id)kSecClassKey,
                                (id)kSecAttrApplicationTag: tag,
                                (id)kSecAttrKeyClass: (id)kSecAttrKeyClassPublic,
                                (id)kSecReturnRef: @YES,
    };
    
    return getquery;
}

+ (NSMutableData *)decrypt:(NSData *)tag cipherText:(NSData *)cipherText withError:(NSError **)errorPtr;
{
    NSDictionary *getPrivateKeyQuery = [KeyAgent createPrivateKeyQuery:tag];
    
    SecKeyRef privateKey = NULL;
    OSStatus status = SecItemCopyMatching((__bridge CFDictionaryRef)getPrivateKeyQuery,
                                          (CFTypeRef *)&privateKey);
    if (status!=errSecSuccess) {
        *errorPtr = [KeyAgent createError:status];
        return nil;
    }
    
    SecKeyAlgorithm algorithm = kSecKeyAlgorithmECIESEncryptionStandardX963SHA256AESGCM;
    
    BOOL canDecrypt = SecKeyIsAlgorithmSupported(privateKey,
                                                 kSecKeyOperationTypeDecrypt,
                                                 algorithm);
    
    NSMutableData* clearText = nil;
    if (canDecrypt) {
        CFErrorRef error = NULL;
        clearText = (NSMutableData*)CFBridgingRelease(       // ARC takes ownership
                                               SecKeyCreateDecryptedData(privateKey,
                                                                         algorithm,
                                                                         (__bridge CFDataRef)cipherText,
                                                                         &error));
        if (!clearText) {
            *errorPtr = CFBridgingRelease(error);  // ARC takes ownership
            return nil;
        }
    }
    
    if (privateKey) { CFRelease(privateKey); }
    
    return clearText;
}

- (instancetype)initWithPublicKey:(SecKeyRef)publicKey {
    self = [super init];
    if (self) {
        _publicKey = publicKey;
    }
    return self;
}

- (NSData *)encrypt:(NSData *)plainText
{
    SecKeyAlgorithm algorithm = kSecKeyAlgorithmECIESEncryptionStandardX963SHA256AESGCM;
    
    BOOL canEncrypt = SecKeyIsAlgorithmSupported(_publicKey,
                                                 kSecKeyOperationTypeEncrypt,
                                                 algorithm);
    
    NSData* cipherText = nil;
    if (canEncrypt) {
        CFErrorRef error = NULL;
        cipherText = (NSData*)CFBridgingRelease(      // ARC takes ownership
                                                SecKeyCreateEncryptedData(_publicKey,
                                                                          algorithm,
                                                                          (__bridge CFDataRef)plainText,
                                                                          &error));
        if (!cipherText) {
            NSError *err = CFBridgingRelease(error);  // ARC takes ownership
            // Handle the error. . .
        }
    }
    
    return cipherText;
}

- (void)dealloc
{
    CFRelease(_publicKey);
}

+ (NSError *)createError:(OSStatus)status
{
    NSString *domain = @"com.example.keys.error";
    NSString *desc = keychainStatusToString(status);
    NSDictionary *userInfo = [[NSDictionary alloc] initWithObjectsAndKeys:desc, @"NSLocalizedDescriptionKey",NULL];
    
    return [NSError errorWithDomain:domain code:-101 userInfo:userInfo];
}

NSString *keychainStatusToString(OSStatus status) {
    NSString *message = [NSString stringWithFormat:@"%ld", (long)status];
    
    switch (status) {
        case errSecSuccess:
            message = @"success";
            break;
            
        case errSecDuplicateItem:
            message = @"error item already exists";
            break;
            
        case errSecItemNotFound :
            message = @"error item not found";
            break;
            
        case errSecAuthFailed:
            message = @"error item authentication failed";
            break;
            
        default:
            message = [NSString stringWithFormat:@"error with OSStatus %d", status];
            break;
    }
    
    return message;
}

@end
