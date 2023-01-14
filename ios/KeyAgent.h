//
//  KeyGen.h
//  RnCardanoWallet
//
//  Created by eliot on 05/01/23.
//  Copyright © 2023 Facebook. All rights reserved.
//

#ifndef KeyAgent_h
#define KeyAgent_h

#import <Foundation/Foundation.h>

@class KeyAgent;

@interface KeyAgent : NSObject

@property (nonatomic, readonly) SecKeyRef publicKey;

+ (KeyAgent *)createKeyAgent:(NSData *)tag withError:(NSError **)errorPtr;
+ (NSMutableData *)decrypt:(NSData *)tag cipherText:(NSData *)cipherText withError:(NSError **)errorPtr;

- (instancetype)initWithPublicKey:(SecKeyRef)publicKey;
- (NSData *) encrypt:(NSData *)plainText;
- (void)dealloc;

@end

#endif /* KeyAgent_h */
