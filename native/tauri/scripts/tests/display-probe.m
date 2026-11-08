#import <AppKit/AppKit.h>
#import <WebKit/WebKit.h>
#include <stdint.h>

extern int32_t euv_configure_apple_display(void *, int32_t, int32_t);

int main(void) {
    @autoreleasepool {
        [NSApplication sharedApplication];
        NSWindow *window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 400, 300)
            styleMask:NSWindowStyleMaskBorderless backing:NSBackingStoreBuffered defer:NO];
        window.releasedWhenClosed = NO;
        WKWebView *web = [[WKWebView alloc] initWithFrame:window.contentLayoutRect];
        window.contentView = web;
        int32_t accepted = euv_configure_apple_display((__bridge void *)web, 1, 1);
        [window close];
        return accepted == 1 ? 0 : 1;
    }
}
