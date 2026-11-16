#import <TargetConditionals.h>
#import <QuartzCore/CADisplayLink.h>
#import <WebKit/WebKit.h>
#import <objc/runtime.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#if TARGET_OS_OSX
#import <AppKit/AppKit.h>
#else
#import <UIKit/UIKit.h>
// These setters belong to Tao's application-owned view controller, not to a
// private UIKit API. Guard each selector before calling the locked Tao ABI.
@interface UIViewController (EuvTaoPreferences)
- (void)setPrefersStatusBarHidden:(BOOL)hidden;
- (void)setPrefersHomeIndicatorAutoHidden:(BOOL)hidden;
@end
#endif

static char displayPolicyKey;

API_AVAILABLE(macos(14.0))
@interface EuvDisplayPolicy : NSObject <WKScriptMessageHandler>
@property(nonatomic, weak) WKWebView *web;
@property(nonatomic, strong) CADisplayLink *displayLink;
@property(nonatomic) BOOL immersive;
@property(nonatomic) BOOL maximumRefresh;
@property(nonatomic) NSInteger requestedRate;
@property(nonatomic) BOOL measuring;
@property(nonatomic, copy) NSString *measuredURL;
@property(nonatomic, copy) NSString *documentToken;
@property(nonatomic, strong) NSData *publishedPolicy;
- (void)refresh:(NSNotification *)notification;
- (void)tick:(CADisplayLink *)link;
@end

// CADisplayLink retains its target. A weak forwarding target avoids keeping
// the policy and its WebView alive after the owning controller is destroyed.
API_AVAILABLE(macos(14.0))
@interface EuvDisplayTarget : NSObject
@property(nonatomic, weak) EuvDisplayPolicy *policy;
- (void)tick:(CADisplayLink *)link;
@end

@implementation EuvDisplayTarget
- (void)tick:(CADisplayLink *)link {
    EuvDisplayPolicy *policy = self.policy;
    if (policy == nil) [link invalidate];
    else [policy tick:link];
}
@end

@implementation EuvDisplayPolicy
- (void)dealloc {
    [self.displayLink invalidate];
    [NSNotificationCenter.defaultCenter removeObserver:self];
}

- (NSInteger)maximumRate {
    return self.web.window.screen.maximumFramesPerSecond;
}

- (BOOL)isActive {
#if TARGET_OS_OSX
    NSWindow *window = self.web.window;
    return NSApp.active && window.visible && !window.miniaturized &&
        (window.occlusionState & NSWindowOcclusionStateVisible) != 0;
#else
    UIWindow *window = self.web.window;
    return window != nil && !window.hidden &&
        window.windowScene.activationState == UISceneActivationStateForegroundActive;
#endif
}

- (void)publish {
    WKWebView *web = self.web;
    if (web.window == nil) return;
    BOOL actualImmersive;
#if TARGET_OS_OSX
    actualImmersive = (web.window.styleMask & NSWindowStyleMaskTitled) == 0;
#else
    UIViewController *controller = web.window.rootViewController;
    actualImmersive = controller.prefersStatusBarHidden;
#endif
    NSDictionary *policy = @{
        @"policy": self.maximumRefresh ? @"maximum" : @"system",
        @"maximumFramesPerSecond": @([self maximumRate]),
        @"requestedFramesPerSecond": @(self.requestedRate),
        @"immersive": @(actualImmersive),
        @"displayLink": @(self.displayLink != nil),
        @"guaranteed": @NO,
    };
    NSError *error = nil;
    NSData *json = [NSJSONSerialization dataWithJSONObject:policy options:NSJSONWritingSortedKeys error:&error];
    if (json == nil) {
        NSLog(@"[EUV] Could not serialize native display policy: %@", error);
        return;
    }
    if ([self.publishedPolicy isEqualToData:json]) return;
    self.publishedPolicy = json;
    __weak EuvDisplayPolicy *weakPolicy = self;
    NSString *value = [[NSString alloc] initWithData:json encoding:NSUTF8StringEncoding];
    NSString *script = [NSString stringWithFormat:@"window.__EUV_NATIVE_DISPLAY__=%@;window.dispatchEvent(new Event('euv:native-display'));", value];
    [web evaluateJavaScript:script completionHandler:^(__unused id result, NSError *evaluationError) {
        if (evaluationError != nil) {
            EuvDisplayPolicy *policy = weakPolicy;
            if (policy.publishedPolicy == json) policy.publishedPolicy = nil;
            NSLog(@"[EUV] Native display policy publication failed: %@", evaluationError.localizedDescription);
        }
    }];
}

- (void)applyImmersive {
#if TARGET_OS_OSX
    if (self.web.window.movableByWindowBackground != self.immersive) {
        self.web.window.movableByWindowBackground = self.immersive;
    }
#else
    UIViewController *controller = self.web.window.rootViewController;
    if (controller == nil) return;
    if (![controller respondsToSelector:@selector(setPrefersStatusBarHidden:)] ||
        ![controller respondsToSelector:@selector(setPrefersHomeIndicatorAutoHidden:)]) {
        NSLog(@"[EUV] Unsupported Tao controller: immersive preferences unavailable");
        return;
    }
    if (controller.prefersStatusBarHidden != self.immersive) {
        [controller setPrefersStatusBarHidden:self.immersive];
    }
    if (controller.prefersHomeIndicatorAutoHidden != self.immersive) {
        [controller setPrefersHomeIndicatorAutoHidden:self.immersive];
    }
#endif
}

- (void)refresh:(__unused NSNotification *)notification {
    WKWebView *web = self.web;
    if (web == nil) {
        [self.displayLink invalidate];
        self.displayLink = nil;
        return;
    }
    [self applyImmersive];
    NSInteger rate = [self maximumRate];
    if (self.maximumRefresh && self.displayLink == nil && web.window != nil) {
        EuvDisplayTarget *target = [EuvDisplayTarget new];
        target.policy = self;
#if TARGET_OS_OSX
        self.displayLink = [web.window displayLinkWithTarget:target selector:@selector(tick:)];
#else
#if defined(__IPHONE_27_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_27_0
        if (@available(iOS 27.0, *)) {
            self.displayLink = [web.window.windowScene displayLinkWithTarget:target selector:@selector(tick:)];
        } else {
            self.displayLink = [CADisplayLink displayLinkWithTarget:target selector:@selector(tick:)];
        }
#else
        self.displayLink = [CADisplayLink displayLinkWithTarget:target selector:@selector(tick:)];
#endif
#endif
        [self.displayLink addToRunLoop:NSRunLoop.mainRunLoop forMode:NSRunLoopCommonModes];
    }
    if (!self.maximumRefresh) {
        [self.displayLink invalidate];
        self.displayLink = nil;
    }
    self.requestedRate = self.maximumRefresh && self.displayLink != nil ? rate : 0;
    if (self.displayLink != nil && rate > 0) {
        // This is an OS preference, not measured WebView FPS. The compositor,
        // accessibility, power and thermal policies retain final authority.
        CAFrameRateRange current = self.displayLink.preferredFrameRateRange;
        if (current.minimum != rate || current.maximum != rate || current.preferred != rate) {
            self.displayLink.preferredFrameRateRange = CAFrameRateRangeMake((float)rate, (float)rate, (float)rate);
        }
    }
    BOOL deactivating = NO;
#if !TARGET_OS_OSX
    deactivating = [notification.name isEqualToString:UIApplicationWillResignActiveNotification] ||
        ([notification.name isEqualToString:UISceneWillDeactivateNotification] &&
         notification.object == web.window.windowScene);
#endif
    self.displayLink.paused = deactivating || ![self isActive];
    [self publish];
    [self measureFrameRate];
}

- (void)tick:(__unused CADisplayLink *)link {
    // Native vsync requests do not replace requestAnimationFrame or synthesize
    // browser frames. Re-read the attached display when its maximum changes.
    if (self.web == nil || self.requestedRate != [self maximumRate]) [self refresh:nil];
}

- (void)measureFrameRate {
    const char *diagnostics = getenv("EUV_CLIENT_DIAGNOSTICS");
    WKWebView *web = self.web;
    if (diagnostics == NULL || strcmp(diagnostics, "1") != 0 || self.measuring ||
        ![web.URL.scheme isEqualToString:@"euv"] ||
        [self.measuredURL isEqualToString:web.URL.absoluteString] || ![self isActive]) return;
    self.measuring = YES;
    NSString *url = web.URL.absoluteString;
    NSString *script = @"if(document.visibilityState!=='visible')throw new Error('Page is not visible');const frame=()=>new Promise(resolve=>requestAnimationFrame(resolve));const start=await frame();let end=start;for(let i=0;i<120;i++){end=await frame();if(document.visibilityState!=='visible')throw new Error('Page became hidden');}return {frames:120,elapsedMs:end-start,framesPerSecond:120000/(end-start),native:window.__EUV_NATIVE_DISPLAY__,gpu:typeof window.__EUV_GPU_PROBE__==='function'?await window.__EUV_GPU_PROBE__():null,url:location.href};";
    __weak EuvDisplayPolicy *weakSelf = self;
    [web callAsyncJavaScript:script arguments:@{} inFrame:nil inContentWorld:WKContentWorld.pageWorld completionHandler:^(id result, NSError *error) {
        EuvDisplayPolicy *policy = weakSelf;
        policy.measuring = NO;
        if (error != nil) {
            NSLog(@"[EUV] Frame-rate sample unavailable: %@", error.localizedDescription);
            return;
        }
        policy.measuredURL = url;
        NSData *data = [NSJSONSerialization dataWithJSONObject:result options:0 error:nil];
        NSLog(@"[EUV] Frame-rate sample: %@", [[NSString alloc] initWithData:data encoding:NSUTF8StringEncoding]);
    }];
}

- (void)userContentController:(__unused WKUserContentController *)controller didReceiveScriptMessage:(WKScriptMessage *)message {
    if (message.frameInfo.mainFrame) {
        NSDictionary *body = [message.body isKindOfClass:NSDictionary.class] ? message.body : nil;
        NSString *token = [body[@"documentToken"] isKindOfClass:NSString.class] ? body[@"documentToken"] : nil;
        if (token != nil && ![token isEqualToString:self.documentToken]) {
            self.documentToken = token;
            self.publishedPolicy = nil;
            self.measuredURL = nil;
        }
        [self refresh:nil];
        [self measureFrameRate];
    }
}
@end

int32_t euv_configure_apple_display(void *pointer, int32_t immersive, int32_t maximumRefresh) {
    @try {
        WKWebView *web = (__bridge WKWebView *)pointer;
        if (web == nil || ![web isKindOfClass:WKWebView.class]) return 0;
        if (@available(macOS 14.0, *)) {
            WKUserContentController *controller = web.configuration.userContentController;
            EuvDisplayPolicy *policy = objc_getAssociatedObject(controller, &displayPolicyKey);
            if (policy == nil) {
                policy = [EuvDisplayPolicy new];
                policy.web = web;
                [controller addScriptMessageHandler:policy name:@"euvNativeDisplay"];
                objc_setAssociatedObject(controller, &displayPolicyKey, policy, OBJC_ASSOCIATION_RETAIN_NONATOMIC);
                NSString *script = @"(function(){if(window.__euvNativeDisplayBridge)return;window.__euvNativeDisplayBridge=true;var pending=false;var documentToken=String(performance.timeOrigin);function refresh(){if(pending)return;pending=true;requestAnimationFrame(function(){pending=false;window.webkit.messageHandlers.euvNativeDisplay.postMessage({documentToken:documentToken});});}document.addEventListener('DOMContentLoaded',refresh);window.addEventListener('load',refresh);window.addEventListener('resize',refresh);document.addEventListener('visibilitychange',refresh);refresh();})();";
                [controller addUserScript:[[WKUserScript alloc] initWithSource:script injectionTime:WKUserScriptInjectionTimeAtDocumentStart forMainFrameOnly:YES]];
                [web evaluateJavaScript:script completionHandler:nil];
                NSNotificationCenter *center = NSNotificationCenter.defaultCenter;
#if TARGET_OS_OSX
                for (NSString *name in @[NSWindowDidChangeScreenNotification, NSWindowDidChangeOcclusionStateNotification,
                        NSApplicationDidBecomeActiveNotification, NSApplicationDidResignActiveNotification,
                        NSApplicationDidChangeScreenParametersNotification]) {
#else
                for (NSString *name in @[UIApplicationDidBecomeActiveNotification, UIApplicationWillResignActiveNotification,
                        UIApplicationDidEnterBackgroundNotification,
                        UISceneDidActivateNotification, UISceneWillDeactivateNotification,
                        UIScreenModeDidChangeNotification, UIWindowDidBecomeVisibleNotification]) {
#endif
                    [center addObserver:policy selector:@selector(refresh:) name:name object:nil];
                }
            }
            policy.immersive = immersive != 0;
            policy.maximumRefresh = maximumRefresh != 0;
            [policy refresh:nil];
            return 1;
        }
        // Older macOS WebKit follows native compositor vsync without a public
        // frame-rate preference API. Do not use private preferences or fake timers.
        NSLog(@"[EUV] Native display cadence follows WebKit compositor vsync on this macOS version");
        return 1;
    } @catch (NSException *error) {
        NSLog(@"[EUV] Native display configuration failed: %@ %@", error.name, error.reason);
        return 0;
    }
}
