# Preserve all classes referenced by AndroidManifest.xml and core app components
-keep class com.euv.MainActivity { *; }
-keep class com.euv.KeepAliveService { *; }
-keep class com.euv.AppConfig { *; }
-keep class com.euv.TauriActivity { *; }
-keep class com.euv.WryActivity { *; }
-keep class com.euv.Rust { *; }
-keep class com.euv.RustWebView { *; }
-keep class com.euv.RustWebViewClient { *; }
-keep class com.euv.RustWebChromeClient { *; }
-keep class com.euv.Ipc { *; }
-keep class com.euv.WryLifecycleObserver { *; }
-keep class com.euv.TauriLifecycleObserver { *; }
-keep class com.euv.Logger { *; }
-keep class com.euv.PermissionHelper { *; }

# Preserve all native method holders
-keepclassmembers class com.euv.** {
    native <methods>;
}

# Preserve JavaScript interface methods
-keepclassmembers class com.euv.Ipc {
    @android.webkit.JavascriptInterface public <methods>;
}

# Tauri plugin manager
-keep class app.tauri.plugin.** { *; }

# Preserve line numbers for debugging
-keepattributes SourceFile,LineNumberTable
-renamesourcefileattribute SourceFile