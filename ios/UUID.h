//
//  UUID.h
//  RnCardanoWallet
//
//  Created by eliot on 06/01/23.
//  Copyright © 2023 Facebook. All rights reserved.
//

#ifndef UUID_h
#define UUID_h

#import <Foundation/Foundation.h>

@class UUID;

@interface UUID : NSObject

+ (NSString *)generate;

@end

#endif /* UUID_h */
