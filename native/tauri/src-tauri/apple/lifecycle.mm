#import <UIKit/UIKit.h>
#import <WebKit/WebKit.h>
#import <objc/runtime.h>
#include <stdint.h>

// Tao 0.35 drops its retained scene configuration before UIKit retains it.
// Preserve lazy class registration without retaining that dangling return.
typedef void *(*SceneConfigurationCallback)(id, SEL, UIApplication *, UISceneSession *, UISceneConnectionOptions *);
static SceneConfigurationCallback originalConfiguration = nullptr;
static char safeAreaHandlerKey;

static void applyNativeSafeArea(WKWebView *web) {
    if (web == nil) return;
    UIEdgeInsets insets = web.window != nil ? web.window.safeAreaInsets : web.safeAreaInsets;
    if (web.scrollView.contentInsetAdjustmentBehavior != UIScrollViewContentInsetAdjustmentNever) {
        web.scrollView.contentInsetAdjustmentBehavior = UIScrollViewContentInsetAdjustmentNever;
        web.scrollView.contentInset = UIEdgeInsetsZero;
        web.scrollView.scrollIndicatorInsets = UIEdgeInsetsZero;
        [web.scrollView setContentOffset:CGPointZero animated:NO];
    }
    NSString *css = [NSString stringWithFormat:@"html{--euv-mobile-safe-top:%gpx!important;--euv-safe-right:%gpx!important;--euv-safe-bottom:%gpx!important;--euv-safe-left:%gpx!important}", insets.top, insets.right, insets.bottom, insets.left];
    NSString *script = [NSString stringWithFormat:@"window.__EUV_NATIVE_SAFE_TOP__=%g;(function(){function apply(){if(!document.head)return;var s=document.getElementById('euv-native-safe-area');if(!s){s=document.createElement('style');s.id='euv-native-safe-area';document.head.appendChild(s);}if(s.textContent!=='%@')s.textContent='%@';}if(document.head)apply();else document.addEventListener('DOMContentLoaded',apply,{once:true});})();", insets.top, css, css];
    [web evaluateJavaScript:script completionHandler:^(__unused id result, NSError *error) {
        if (error != nil) NSLog(@"[EUV] Native safe-area update failed: %@", error.localizedDescription);
    }];
}

@interface EuvSafeAreaHandler : NSObject <WKScriptMessageHandler>
@end

@implementation EuvSafeAreaHandler
- (void)userContentController:(__unused WKUserContentController *)controller didReceiveScriptMessage:(WKScriptMessage *)message {
    applyNativeSafeArea(message.webView);
}
@end

static UISceneConfiguration *sceneConfiguration(id delegate, SEL selector, UIApplication *application,
                                               UISceneSession *session, UISceneConnectionOptions *options) {
    if (originalConfiguration != nullptr) {
        (void)originalConfiguration(delegate, selector, application, session, options);
    }
    Class sceneDelegate = NSClassFromString(@"TaoSceneDelegate");
    if (sceneDelegate == Nil) {
        NSLog(@"[EUV] Tao scene delegate class was not registered");
        return nil;
    }
    UISceneConfiguration *configuration = [[UISceneConfiguration alloc] initWithName:@"TaoScene" sessionRole:session.role];
    configuration.delegateClass = sceneDelegate;
    return configuration;
}

extern "C" int32_t euv_install_scene_lifetime_fix(void) {
    if (originalConfiguration != nullptr) return 1;
    Class delegate = NSClassFromString(@"AppDelegate");
    SEL selector = @selector(application:configurationForConnectingSceneSession:options:);
    Method method = delegate != Nil ? class_getInstanceMethod(delegate, selector) : nullptr;
    if (method == nullptr) return 0;
    originalConfiguration = reinterpret_cast<SceneConfigurationCallback>(method_getImplementation(method));
    class_replaceMethod(delegate, selector, reinterpret_cast<IMP>(&sceneConfiguration), method_getTypeEncoding(method));
    return 1;
}

extern "C" int32_t euv_configure_ios_webview(void *pointer) {
    WKWebView *web = (__bridge WKWebView *)pointer;
    if (web == nil || ![web isKindOfClass:[WKWebView class]]) return 0;
    WKUserContentController *controller = web.configuration.userContentController;
    NSString *script = @"(function(){if(window.__euvNativeSafeAreaBridge)return;window.__euvNativeSafeAreaBridge=true;function refresh(){window.webkit.messageHandlers.euvNativeSafeArea.postMessage(null);}document.addEventListener('DOMContentLoaded',refresh);window.addEventListener('load',refresh);window.addEventListener('resize',refresh);if(window.visualViewport)window.visualViewport.addEventListener('resize',refresh);refresh();})();";
    if (objc_getAssociatedObject(controller, &safeAreaHandlerKey) == nil) {
        EuvSafeAreaHandler *handler = [EuvSafeAreaHandler new];
        [controller addScriptMessageHandler:handler name:@"euvNativeSafeArea"];
        objc_setAssociatedObject(controller, &safeAreaHandlerKey, handler, OBJC_ASSOCIATION_RETAIN_NONATOMIC);
        [controller addUserScript:[[WKUserScript alloc] initWithSource:script injectionTime:WKUserScriptInjectionTimeAtDocumentStart forMainFrameOnly:YES]];
    }
    [web evaluateJavaScript:script completionHandler:^(__unused id result, NSError *error) {
        if (error != nil) NSLog(@"[EUV] Native safe-area bridge setup failed: %@", error.localizedDescription);
    }];
    applyNativeSafeArea(web);
    return 1;
}
