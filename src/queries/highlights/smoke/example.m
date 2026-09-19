// café: Objective-C is highlighted, never executed
#import <Foundation/Foundation.h>
struct Pair { int field; };
static int twice(int ordinary) { return ordinary * 2; }
@protocol Greeting
- (NSString *)greet:(NSString *)person;
@end
@interface Example : NSObject <Greeting>
@property(nonatomic, strong) NSString *title;
+ (instancetype)shared;
@end
@implementation Example
@synthesize title;
+ (instancetype)shared { return [[self alloc] init]; }
- (NSString *)greet:(NSString *)person {
    int ordinary = 42;
    NSNumber *boxed = @17;
    NSArray *items = @[@"hello", boxed];
    NSDictionary *mapping = @{@"key": items};
    void (^callback)(void) = ^{ NSLog(@"café"); };
    SEL selector = @selector(description);
    @try {
        if (ordinary > 0) { [super description]; callback(); }
    } @catch (NSException *exception) {
        @throw exception;
    } @finally { ordinary = twice(ordinary); }
    /* a block comment */
    return [person uppercaseString];
}
@end
int after(void) { return 99; }
