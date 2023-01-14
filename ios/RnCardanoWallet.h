// #ifdef __cplusplus
// #import "rn-cardano-wallet.h"
// #endif

#ifdef RCT_NEW_ARCH_ENABLED
#import "RNRnCardanoWalletSpec.h"

@interface RnCardanoWallet : NSObject <NativeRnCardanoWalletSpec>
#else
#import <React/RCTBridgeModule.h>

@interface RnCardanoWallet : NSObject <RCTBridgeModule>
#endif

@end
