package com.euv

import android.Manifest
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.content.res.Configuration
import android.graphics.Color
import android.graphics.drawable.ColorDrawable
import android.hardware.display.DisplayManager
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.provider.Settings
import android.util.Log
import android.view.Display
import android.view.Gravity
import android.view.View
import android.view.ViewGroup
import android.view.WindowManager
import android.webkit.JavascriptInterface
import android.webkit.WebView
import android.webkit.WebChromeClient
import android.webkit.ConsoleMessage
import android.widget.FrameLayout
import android.widget.ImageView
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.content.ContextCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

class MainActivity : TauriActivity() {

    private var splashView: View? = null
    private var mainWebView: WebView? = null
    private var pendingPermissionCallback: String? = null
    private var pendingPermissionResults: MutableMap<String, Boolean> = mutableMapOf()
    private var pendingPermissionsToRequest: Array<String> = emptyArray()
    private var lastDispatchedImeBottom: Int = -1
    private var displayListener: DisplayManager.DisplayListener? = null
    private val mainHandler = Handler(Looper.getMainLooper())

    private val multiPermissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { results: Map<String, Boolean> ->
        Log.d("EUV_PERM", "Permission results: $results")
        pendingPermissionResults.putAll(results)
        val allProcessed = pendingPermissionsToRequest.all { perm ->
            pendingPermissionResults.containsKey(perm)
        }
        if (allProcessed && pendingPermissionCallback != null && mainWebView != null) {
            val callbackId = pendingPermissionCallback!!
            val jsResults = pendingPermissionResults.entries.joinToString(",") { (perm, granted) ->
                "{\"permission\":\"$perm\",\"granted\":$granted}"
            }
            val js = "window.__euvBridgeCallback && window.__euvBridgeCallback('$callbackId',[$jsResults])"
            mainWebView?.post {
                mainWebView?.evaluateJavascript(js, null)
            }
            pendingPermissionCallback = null
            pendingPermissionResults.clear()
        }
    }

    private val notificationPermissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestPermission()
    ) { granted: Boolean ->
        if (granted) {
            startKeepAliveService()
        } else {
            Log.w("EUV_CACHE", "POST_NOTIFICATIONS permission denied, keep-alive service not started")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        Log.d("EUV_CACHE", "MainActivity.onCreate()")
        try {
            val field = RustWebViewClient::class.java.getDeclaredField("mainFrameLoaded")
            field.isAccessible = true
            field.set(null, false)
            Log.d("EUV_CACHE", "Reset mainFrameLoaded via reflection")
        } catch (e: Exception) {
            Log.d("EUV_CACHE", "mainFrameLoaded field not found (expected on vanilla Tauri)")
        }
        window.setBackgroundDrawable(ColorDrawable(Color.parseColor(AppConfig.BACKGROUND_COLOR)))
        // NOTE: do NOT call window.setSoftInputMode(SOFT_INPUT_ADJUST_RESIZE) —
        // it has no effect in edge-to-edge mode (setDecorFitsSystemWindows(false)).
        // The IME inset is dispatched manually in onWebViewCreate via
        // setOnApplyWindowInsetsListener on the WebView itself, where we shrink
        // its bottomMargin so the focused input scrolls above the keyboard.
        super.onCreate(savedInstanceState)
        addSplashOverlay()
        if (AppConfig.IMMERSIVE_MODE) {
            enableImmersiveMode()
        } else if (AppConfig.TRANSPARENT_NAVIGATION_BAR) {
            enableTransparentNavigationBar()
        }
        setMaxFrameRate()
        if (AppConfig.MAX_FRAME_RATE_ENABLED) {
            registerDisplayListener()
        }
        if (AppConfig.KEEP_ALIVE_SERVICE) {
            startKeepAliveServiceSafely()
        }
    }

    private fun startKeepAliveServiceSafely() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            when {
                ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS) == PackageManager.PERMISSION_GRANTED -> {
                    startKeepAliveService()
                }
                shouldShowRequestPermissionRationale(Manifest.permission.POST_NOTIFICATIONS) -> {
                    Log.w("EUV_CACHE", "User previously denied POST_NOTIFICATIONS, requesting again")
                    notificationPermissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS)
                }
                else -> {
                    notificationPermissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS)
                }
            }
        } else {
            startKeepAliveService()
        }
    }

    private fun startKeepAliveService() {
        val serviceIntent = Intent(this, KeepAliveService::class.java)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            startForegroundService(serviceIntent)
        } else {
            startService(serviceIntent)
        }
        Log.d("EUV_CACHE", "KeepAliveService started")
    }

    private fun addSplashOverlay() {
        val rootView = window.decorView as ViewGroup
        val splash = FrameLayout(this).apply {
            setBackgroundColor(Color.parseColor(AppConfig.BACKGROUND_COLOR))
            layoutParams = FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            )
            elevation = 100f
            val logo = ImageView(context).apply {
                setImageResource(R.mipmap.ic_launcher)
                layoutParams = FrameLayout.LayoutParams(
                    240, 240, Gravity.CENTER
                )
                scaleType = ImageView.ScaleType.FIT_CENTER
            }
            addView(logo)
        }
        rootView.addView(splash)
        splashView = splash
        Log.d("EUV_CACHE", "Splash overlay added to DecorView")
    }

    fun removeSplash() {
        splashView?.let { splash ->
            splash.animate()
                .alpha(0f)
                .setDuration(AppConfig.SPLASH_FADE_DURATION_MS)
                .withEndAction {
                    (splash.parent as? ViewGroup)?.removeView(splash)
                    splashView = null
                    Log.d("EUV_CACHE", "Splash removed")
                }
                .start()
        }
    }

    private fun enableImmersiveMode() {
        // Edge-to-edge: the app draws behind the status and navigation bars.
        // WindowCompat routes to Window.setDecorFitsSystemWindows(false) on
        // API 30+ and to the equivalent systemUiVisibility flags below.
        WindowCompat.setDecorFitsSystemWindows(window, false)
        val controller = WindowInsetsControllerCompat(window, window.decorView)
        // Hide both bars. BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE keeps them
        // reachable with a swipe, and they auto-hide again afterwards.
        controller.hide(WindowInsetsCompat.Type.systemBars())
        controller.systemBarsBehavior =
            WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R) {
            // Pre-R the controller above is backed by systemUiVisibility; the
            // sticky flags keep the bars hidden after transient swipe reveals.
            @Suppress("DEPRECATION")
            window.decorView.systemUiVisibility = (
                View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                or View.SYSTEM_UI_FLAG_FULLSCREEN
                or View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                or View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                or View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
            )
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            window.attributes.layoutInDisplayCutoutMode =
                WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
        }
    }

    private fun enableTransparentNavigationBar() {
        WindowCompat.setDecorFitsSystemWindows(window, false)
        window.navigationBarColor = android.graphics.Color.TRANSPARENT
        window.statusBarColor = android.graphics.Color.TRANSPARENT
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            window.setDecorFitsSystemWindows(false)
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            window.attributes.layoutInDisplayCutoutMode =
                WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
        }
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R) {
            @Suppress("DEPRECATION")
            window.decorView.systemUiVisibility = (
                View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                or View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
            )
        }
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        if (hasFocus && AppConfig.IMMERSIVE_MODE) {
            enableImmersiveMode()
        } else if (hasFocus && AppConfig.TRANSPARENT_NAVIGATION_BAR) {
            enableTransparentNavigationBar()
        }
        if (hasFocus) {
            // Focus can be regained after a transient bar reveal, a notification
            // shade pull, or a return from another app; re-assert the refresh
            // request because the surface may have been reset meanwhile.
            setMaxFrameRate()
        }
    }

    override fun onResume() {
        super.onResume()
        // Coming back from the background: re-hide the bars and re-assert the
        // refresh rate, because the system resets the preferred display mode
        // when the activity is no longer visible.
        if (AppConfig.IMMERSIVE_MODE) {
            enableImmersiveMode()
        } else if (AppConfig.TRANSPARENT_NAVIGATION_BAR) {
            enableTransparentNavigationBar()
        }
        setMaxFrameRate()
    }

    override fun onConfigurationChanged(newConfig: Configuration) {
        super.onConfigurationChanged(newConfig)
        // Rotation, fold/unfold and display moves can change the active
        // display and its supported modes; re-apply the immersive window and
        // the refresh request for the new display.
        if (AppConfig.IMMERSIVE_MODE) {
            enableImmersiveMode()
        }
        setMaxFrameRate()
    }

    override fun onDestroy() {
        // Symmetric teardown: never leave a DisplayListener registered.
        unregisterDisplayListener()
        mainHandler.removeCallbacksAndMessages(null)
        super.onDestroy()
    }

    /**
     * Resolve the display the window is currently attached to.
     *
     * `Activity.getDisplay()` only exists from API 30, so the window's decor
     * view is used as the fallback on older releases. Returns null when the
     * activity is not attached to a window yet.
     */
    private fun currentDisplay(): Display? {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            return display
        }
        @Suppress("DEPRECATION")
        return windowManager.defaultDisplay
    }

    /**
     * Highest refresh rate the given display supports, derived from the
     * display's own supported modes. No refresh rate is hardcoded: the value
     * always reflects the hardware attached to the current display.
     */
    private fun highestRefreshRate(display: Display): Float {
        val modes = display.supportedModes
        if (modes.isNotEmpty()) {
            // Only consider modes with the current physical resolution so a
            // resolution change is never requested as a side effect.
            val currentMode = display.mode
            val sameSizeModes = modes.filter {
                it.physicalWidth == currentMode.physicalWidth &&
                    it.physicalHeight == currentMode.physicalHeight
            }
            return sameSizeModes.maxOfOrNull { it.refreshRate } ?: currentMode.refreshRate
        }
        // Fall back to the advertised refresh rates when modes are unavailable.
        val rates = display.supportedRefreshRates
        return rates.maxOrNull() ?: display.refreshRate
    }

    /**
     * Ask the platform for the highest refresh rate available on the current
     * display.
     *
     * Only public APIs are used:
     *  - `WindowManager.LayoutParams.preferredRefreshRate` asks the platform
     *    for a refresh rate without pinning a display mode, so the resolution
     *    is never changed (unlike `preferredDisplayModeId`, which selects a
     *    whole mode and can change the resolution).
     *  - `View.setRequestedFrameRate` lets the view hierarchy vote for a
     *    frame rate on Android 15+.
     *
     * The platform treats these as preferences: battery saver, thermal
     * throttling and the display's own power policy can still lower the
     * effective refresh rate.
     */
    private fun setMaxFrameRate() {
        if (!AppConfig.MAX_FRAME_RATE_ENABLED) return
        val display = currentDisplay()
        if (display == null) {
            Log.d("EUV_CACHE", "setMaxFrameRate: no display attached yet")
            return
        }
        val refreshRate = highestRefreshRate(display)
        if (refreshRate <= 0f) {
            Log.d("EUV_CACHE", "setMaxFrameRate: display reports no usable refresh rate")
            return
        }
        try {
            val params: WindowManager.LayoutParams = window.attributes
            // preferredRefreshRate keeps the current display mode (and hence the
            // current resolution); preferredDisplayModeId is deliberately not
            // used because it can select a mode with a different resolution.
            params.preferredRefreshRate = refreshRate
            params.preferredDisplayModeId = 0
            window.attributes = params
            Log.d(
                "EUV_CACHE",
                "Requested refresh rate ${refreshRate}Hz on display ${display.displayId} " +
                    "(mode ${display.mode.modeId})"
            )
        } catch (e: Exception) {
            Log.w("EUV_CACHE", "Failed to set preferredRefreshRate: ${e.message}")
        }
        applyViewFrameRateVote(refreshRate)
    }

    /**
     * Vote for the highest refresh rate from the view hierarchy.
     *
     * `View.setRequestedFrameRate` is available from API 35 (Android 15). The
     * vote is advisory: the framework combines it with votes from other views
     * and the display decides the final rate.
     */
    private fun applyViewFrameRateVote(refreshRate: Float) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.VANILLA_ICE_CREAM) return
        try {
            val decorView = window.decorView
            decorView.setRequestedFrameRate(refreshRate)
            Log.d("EUV_CACHE", "View frame-rate vote set to ${refreshRate}Hz")
        } catch (e: Exception) {
            Log.w("EUV_CACHE", "Failed to set view frame rate vote: ${e.message}")
        }
    }

    /**
     * Register a DisplayManager listener so display changes (rotation, an
     * external display being attached, or the activity being moved to another
     * display on a foldable) re-apply the refresh request.
     *
     * Registration and unregistration are symmetric: the listener is only
     * registered once and always removed in `onDestroy`.
     */
    private fun registerDisplayListener() {
        if (displayListener != null) return
        val displayManager =
            getSystemService(Context.DISPLAY_SERVICE) as? DisplayManager ?: return
        val listener = object : DisplayManager.DisplayListener {
            override fun onDisplayAdded(displayId: Int) {
                Log.d("EUV_CACHE", "Display added: $displayId")
                mainHandler.post { setMaxFrameRate() }
            }

            override fun onDisplayChanged(displayId: Int) {
                Log.d("EUV_CACHE", "Display changed: $displayId")
                mainHandler.post { setMaxFrameRate() }
            }

            override fun onDisplayRemoved(displayId: Int) {
                Log.d("EUV_CACHE", "Display removed: $displayId")
                mainHandler.post { setMaxFrameRate() }
            }
        }
        try {
            displayManager.registerDisplayListener(listener, mainHandler)
            displayListener = listener
            Log.d("EUV_CACHE", "DisplayManager listener registered")
        } catch (e: Exception) {
            Log.w("EUV_CACHE", "Failed to register DisplayManager listener: ${e.message}")
        }
    }

    private fun unregisterDisplayListener() {
        val listener = displayListener ?: return
        displayListener = null
        try {
            val displayManager =
                getSystemService(Context.DISPLAY_SERVICE) as? DisplayManager
            displayManager?.unregisterDisplayListener(listener)
            Log.d("EUV_CACHE", "DisplayManager listener unregistered")
        } catch (e: Exception) {
            Log.w("EUV_CACHE", "Failed to unregister DisplayManager listener: ${e.message}")
        }
    }

    inner class ExternalLinkHandler {
        @JavascriptInterface
        fun openUrl(url: String) {
            Log.d("EUV_CACHE", "Opening external link: $url")
            try {
                val intent = Intent(Intent.ACTION_VIEW, Uri.parse(url))
                startActivity(intent)
            } catch (e: Exception) {
                Log.e("EUV_CACHE", "Failed to open external link: $url", e)
            }
        }
    }

    inner class EuvBridgeNative {
        @JavascriptInterface
        fun check(permission: String): Boolean {
            return ContextCompat.checkSelfPermission(
                this@MainActivity, permission
            ) == PackageManager.PERMISSION_GRANTED
        }

        @JavascriptInterface
        fun checkMultiple(permissionsJson: String): String {
            val permissions: Array<String> = try {
                val jsonArray = org.json.JSONArray(permissionsJson)
                (0 until jsonArray.length()).map { jsonArray.getString(it) }.toTypedArray()
            } catch (e: Exception) {
                Log.e("EUV_PERM", "Failed to parse permissions JSON: $permissionsJson", e)
                return "[]"
            }
            val results = permissions.map { perm ->
                val granted = ContextCompat.checkSelfPermission(
                    this@MainActivity, perm
                ) == PackageManager.PERMISSION_GRANTED
                "{\"permission\":\"$perm\",\"granted\":$granted}"
            }
            return "[${results.joinToString(",")}]"
        }

        @JavascriptInterface
        fun request(permissionsJson: String, callbackId: String) {
            val permissions: Array<String> = try {
                val jsonArray = org.json.JSONArray(permissionsJson)
                (0 until jsonArray.length()).map { jsonArray.getString(it) }.toTypedArray()
            } catch (e: Exception) {
                Log.e("EUV_PERM", "Failed to parse permissions JSON: $permissionsJson", e)
                if (mainWebView != null) {
                    val js = "window.__euvBridgeCallback && window.__euvBridgeCallback('$callbackId',[])"
                    mainWebView?.post {
                        mainWebView?.evaluateJavascript(js, null)
                    }
                }
                return
            }
            val alreadyGranted = permissions.filter { perm ->
                ContextCompat.checkSelfPermission(
                    this@MainActivity, perm
                ) == PackageManager.PERMISSION_GRANTED
            }
            val toRequest = permissions.filter { perm ->
                ContextCompat.checkSelfPermission(
                    this@MainActivity, perm
                ) != PackageManager.PERMISSION_GRANTED
            }
            if (toRequest.isEmpty()) {
                val results = permissions.map { perm ->
                    "{\"permission\":\"$perm\",\"granted\":true}"
                }
                val js = "window.__euvBridgeCallback && window.__euvBridgeCallback('$callbackId',[${results.joinToString(",")}])"
                mainWebView?.post {
                    mainWebView?.evaluateJavascript(js, null)
                }
                return
            }
            pendingPermissionCallback = callbackId
            pendingPermissionResults.clear()
            pendingPermissionResults.putAll(alreadyGranted.associateWith { true })
            pendingPermissionsToRequest = permissions
            runOnUiThread {
                multiPermissionLauncher.launch(toRequest.toTypedArray())
            }
        }

        @JavascriptInterface
        fun requestGroup(group: String, callbackId: String) {
            val permissions = PermissionGroups.getPermissions(group)
            if (permissions.isEmpty()) {
                val js = "window.__euvBridgeCallback && window.__euvBridgeCallback('$callbackId',[])"
                mainWebView?.post {
                    mainWebView?.evaluateJavascript(js, null)
                }
                return
            }
            val permissionsJson = org.json.JSONArray(permissions).toString()
            request(permissionsJson, callbackId)
        }

        @JavascriptInterface
        fun checkGroup(group: String): String {
            val permissions = PermissionGroups.getPermissions(group)
            val results = permissions.map { perm ->
                val granted = ContextCompat.checkSelfPermission(
                    this@MainActivity, perm
                ) == PackageManager.PERMISSION_GRANTED
                "{\"permission\":\"$perm\",\"granted\":$granted}"
            }
            return "[${results.joinToString(",")}]"
        }

        @JavascriptInterface
        fun checkAll(): String {
            val permissions = PermissionGroups.getPermissions("all")
            val results = permissions.map { perm ->
                val granted = ContextCompat.checkSelfPermission(
                    this@MainActivity, perm
                ) == PackageManager.PERMISSION_GRANTED
                "{\"permission\":\"$perm\",\"granted\":$granted}"
            }
            return "[${results.joinToString(",")}]"
        }

        @JavascriptInterface
        fun requestAll(callbackId: String) {
            requestGroup("all", callbackId)
        }

        @JavascriptInterface
        fun openSettings() {
            runOnUiThread {
                try {
                    val intent = Intent(Settings.ACTION_APPLICATION_DETAILS_SETTINGS)
                    intent.data = Uri.fromParts("package", this@MainActivity.packageName, null)
                    this@MainActivity.startActivity(intent)
                } catch (e: Exception) {
                    Log.e("EUV_PERM", "Failed to open app settings", e)
                }
            }
        }
    }

    override fun onWebViewCreate(webView: WebView) {
        Log.d("EUV_CACHE", "onWebViewCreate() at ${System.currentTimeMillis()}")
        super.onWebViewCreate(webView)
        mainWebView = webView
        WebView.setWebContentsDebuggingEnabled(BuildConfig.DEBUG)
        webView.settings.apply {
            @Suppress("DEPRECATION")
            allowFileAccessFromFileURLs = true
            @Suppress("DEPRECATION")
            allowUniversalAccessFromFileURLs = true
            allowFileAccess = true
            domStorageEnabled = true
            javaScriptEnabled = true
            @Suppress("DEPRECATION")
            layoutAlgorithm = android.webkit.WebSettings.LayoutAlgorithm.NORMAL
            blockNetworkImage = false
            loadsImagesAutomatically = true
            cacheMode = android.webkit.WebSettings.LOAD_DEFAULT
        }
        // The application/window enables GPU acceleration. NONE keeps the
        // accelerated WebView compositor without an extra full-window texture.
        webView.setLayerType(View.LAYER_TYPE_NONE, null)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            webView.setRendererPriorityPolicy(WebView.RENDERER_PRIORITY_IMPORTANT, true)
        }
        if (webView.isAttachedToWindow) {
            Log.i("EUV_RENDERER", "hardwareAccelerated=${webView.isHardwareAccelerated} layerType=${webView.layerType}")
        } else {
            webView.addOnAttachStateChangeListener(object : View.OnAttachStateChangeListener {
                override fun onViewAttachedToWindow(view: View) {
                    Log.i("EUV_RENDERER", "hardwareAccelerated=${view.isHardwareAccelerated} layerType=${view.layerType}")
                    view.removeOnAttachStateChangeListener(this)
                }
                override fun onViewDetachedFromWindow(view: View) = Unit
            })
        }
        webView.setBackgroundColor(Color.parseColor(AppConfig.BACKGROUND_COLOR))
        // Edge-to-edge IME bridge. The activity opted into
        // `setDecorFitsSystemWindows(false)` (see `enableTransparentNavigationBar`),
        // so the default soft-input path (where the system resizes the WebView
        // for us) does NOT fire. On API >= R we know the real IME inset and
        // shrink the WebView's bottomMargin to it, which is the only reliable
        // way to make `window.visualViewport.height()` reflect the keyboard
        // (without that, the framework's `on_focus_scroll_into_view` cannot
        // compute a correct `visible_bottom` and never scrolls the focused
        // input above the keyboard).
        //
        // We also publish the same value as `--euv-keyboard-height` for the
        // framework's `c_euv_input_wrapper` and the inline-style bridge in
        // euv-app's `IMMERSIVE_SAFE_AREA_SCRIPT`.
        //
        // On API < R there is no IME-aware inset API; in that case we fall back
        // to the JS-only bridge (visualViewport resize event), which is good
        // enough for older devices.
        webView.setOnApplyWindowInsetsListener { _, insets ->
            val imeBottom: Int =
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
                    val compatInsets = WindowInsetsCompat.toWindowInsetsCompat(insets, webView)
                    val ime = compatInsets.getInsets(WindowInsetsCompat.Type.ime())
                    val sysBars = compatInsets.getInsets(WindowInsetsCompat.Type.systemBars())
                    // On API >= R the IME inset already includes the
                    // navigation bar height when the IME is up, so taking
                    // max(ime, systemBars) here would double-count. When the
                    // IME is hidden, ime.bottom is 0 and sysBars.bottom (the
                    // nav bar) shows through the transparent navigation bar
                    // background — but the WebView must NOT consume it as a
                    // margin because that would hide the top safe area. We
                    // therefore use ime.bottom only.
                    ime.bottom
                } else {
                    0
                }
            // Shrink the WebView to leave room for the keyboard. Without this,
            // `window.visualViewport.height()` stays at full-screen even when
            // the IME covers the bottom half, and the framework's focus-scroll
            // logic computes a `visible_bottom` that is >= the input's bottom
            // edge — so it never scrolls, and the input stays under the
            // keyboard. This margin is the only signal that actually moves the
            // layout viewport.
            val params: ViewGroup.LayoutParams = webView.layoutParams
            if (params is ViewGroup.MarginLayoutParams) {
                if (params.bottomMargin != imeBottom) {
                    params.bottomMargin = imeBottom
                    webView.layoutParams = params
                }
            }
            // Skip the JS round-trip if the IME height hasn't changed since the
            // last dispatch — repeated identical values would otherwise force a
            // CSS var reassignment + style invalidation on every input focus
            // blur, which visibly shakes the euv example layout.
            //
            // The JS value is published in CSS pixels: `ime.bottom` is in the
            // WebView's physical pixels, while `--euv-keyboard-height` and
            // `window.__EUV_KEYBOARD_HEIGHT__` are consumed by CSS / page JS.
            if (imeBottom != lastDispatchedImeBottom) {
                lastDispatchedImeBottom = imeBottom
                val density: Float = webView.resources.displayMetrics.density
                val cssKeyboardHeight: Int =
                    if (density > 0f) Math.round(imeBottom / density) else imeBottom
                webView.evaluateJavascript(
                    "(function(){var px=${cssKeyboardHeight};" +
                        "document.documentElement.style.setProperty(" +
                        "'--euv-keyboard-height',px+'px');" +
                        "window.__EUV_KEYBOARD_HEIGHT__=px;" +
                        "window.dispatchEvent(new CustomEvent('euv:keyboard-height'," +
                        "{detail:{height:px}}))})();",
                    null,
                )
            }
            // IMPORTANT: hand the insets to the WebView's own implementation
            // instead of returning them raw. Setting this listener REPLACES
            // WebView.onApplyWindowInsets in the dispatch chain, and Chromium's
            // implementation is what feeds the system-bar insets into the
            // page's CSS `env(safe-area-inset-*)` values. Returning `insets`
            // directly starves the page of insets entirely: env() reads 0px,
            // every safe-area probe measures 0, and the mobile header glues
            // itself to the screen top under the status bar. Calling through
            // keeps CSS env() working while we observe the IME inset.
            webView.onApplyWindowInsets(insets)
        }
        webView.webChromeClient = object : WebChromeClient() {
            override fun onConsoleMessage(msg: ConsoleMessage): Boolean {
                Log.d("JSConsole", "${msg.message()} [line ${msg.lineNumber()}]")
                return true
            }
        }
        if (AppConfig.ANTI_ALIASING) {
            webView.evaluateJavascript("""
                (function () {
                    var style = document.createElement('style');
                    style.textContent = '
                        * { -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale; text-rendering: optimizeLegibility; }
                        canvas { image-rendering: auto; }
                    ';
                    document.head.appendChild(style);
                })();
            """.trimIndent(), null)
        }
        webView.addJavascriptInterface(ExternalLinkHandler(), "NativeApp")
        webView.addJavascriptInterface(EuvBridgeNative(), "EuvBridgeNative")
        injectBridgeScript(webView)
        val handler = Handler(Looper.getMainLooper())
        val pollRunnable = object : Runnable {
            override fun run() {
                val currentUrl = webView.url ?: ""
                if (currentUrl.contains("euv.localhost")) {
                    webView.evaluateJavascript(
                        "(document.querySelector('canvas') !== null || document.body.children.length > 1).toString()"
                    ) { result ->
                        val rendered = result?.trim('"') == "true"
                        Log.d("EUV_CACHE", "Splash poll: url=$currentUrl, rendered=$rendered")
                        if (rendered) {
                            Log.d("EUV_CACHE", "WASM app rendered, removing splash")
                            removeSplash()
                            injectExternalLinkInterceptor(webView)
                        } else {
                            handler.postDelayed(this, 32)
                        }
                    }
                } else {
                    handler.postDelayed(this, 50)
                }
            }
        }
        handler.post(pollRunnable)
        Log.d("EUV_CACHE", "WebView setup done at ${System.currentTimeMillis()}")
    }

    private fun injectBridgeScript(webView: WebView) {
        val js = """
            (function () {
                if (window.__euvBridgeInstalled) return;
                window.__euvBridgeInstalled = true;
                window.__euvBridgeCallbacks = {};
                window.__euvBridgeCallbackId = 0;

                window.__euvBridgeCallback = function(callbackId, results) {
                    var cb = window.__euvBridgeCallbacks[callbackId];
                    if (cb) {
                        delete window.__euvBridgeCallbacks[callbackId];
                        cb(results);
                    }
                };

                window.EuvBridge = {
                    check: function(permission) {
                        if (!window.EuvBridgeNative) return { permission: permission, granted: true };
                        var granted = window.EuvBridgeNative.check(permission);
                        return { permission: permission, granted: granted };
                    },
                    checkMultiple: function(permissions) {
                        if (!window.EuvBridgeNative) return permissions.map(function(p) { return { permission: p, granted: true }; });
                        var json = JSON.stringify(permissions);
                        var resultStr = window.EuvBridgeNative.checkMultiple(json);
                        try { return JSON.parse(resultStr); } catch(e) { return []; }
                    },
                    request: function(permissions) {
                        return new Promise(function(resolve) {
                            if (!window.EuvBridgeNative) { resolve(permissions.map(function(p) { return { permission: p, granted: true }; })); return; }
                            var callbackId = '__perm_' + (++window.__euvBridgeCallbackId);
                            window.__euvBridgeCallbacks[callbackId] = resolve;
                            var json = JSON.stringify(permissions);
                            window.EuvBridgeNative.request(json, callbackId);
                        });
                    },
                    requestGroup: function(group) {
                        return new Promise(function(resolve) {
                            if (!window.EuvBridgeNative) { resolve([]); return; }
                            var callbackId = '__perm_' + (++window.__euvBridgeCallbackId);
                            window.__euvBridgeCallbacks[callbackId] = resolve;
                            window.EuvBridgeNative.requestGroup(group, callbackId);
                        });
                    },
                    checkGroup: function(group) {
                        if (!window.EuvBridgeNative) return [];
                        var resultStr = window.EuvBridgeNative.checkGroup(group);
                        try { return JSON.parse(resultStr); } catch(e) { return []; }
                    },
                    checkAll: function() {
                        if (!window.EuvBridgeNative) return [];
                        var resultStr = window.EuvBridgeNative.checkAll();
                        try { return JSON.parse(resultStr); } catch(e) { return []; }
                    },
                    requestAll: function() {
                        return new Promise(function(resolve) {
                            if (!window.EuvBridgeNative) { resolve([]); return; }
                            var callbackId = '__perm_' + (++window.__euvBridgeCallbackId);
                            window.__euvBridgeCallbacks[callbackId] = resolve;
                            window.EuvBridgeNative.requestAll(callbackId);
                        });
                    },
                    openSettings: function() {
                        if (window.EuvBridgeNative) window.EuvBridgeNative.openSettings();
                    }
                };
                console.log('[EUV] Bridge installed');
            })();
        """.trimIndent()
        webView.evaluateJavascript(js, null)
        Log.d("EUV_CACHE", "Bridge JS injected")
    }

    private fun injectExternalLinkInterceptor(webView: WebView) {
        val js = """
            (function () {
                if (window.__externalLinkInterceptorInstalled) return;
                window.__externalLinkInterceptorInstalled = true;
                document.addEventListener('click', function(e) {
                    var target = e.target;
                    while (target && target.tagName !== 'A') {
                        target = target.parentElement;
                    }
                    if (target && target.href) {
                        var href = target.href;
                        if (href.indexOf('euv.localhost') === -1 &&
                            href.indexOf('euv://') !== 0 &&
                            href.indexOf('tauri.localhost') === -1 &&
                            (href.indexOf('http://') === 0 || href.indexOf('https://') === 0)) {
                            e.preventDefault();
                            e.stopPropagation();
                            window.NativeApp.openUrl(href);
                        }
                    }
                }, true);
                var originalOpen = window.open;
                window.open = function(url, target, features) {
                    if (url && url.indexOf('http') === 0 &&
                        url.indexOf('euv.localhost') === -1 &&
                        url.indexOf('tauri.localhost') === -1) {
                        window.NativeApp.openUrl(url);
                        return null;
                    }
                    return originalOpen.call(window, url, target, features);
                };
                console.log('[EUV] External link interceptor installed');
            })();
        """.trimIndent()
        webView.evaluateJavascript(js, null)
        Log.d("EUV_CACHE", "External link interceptor JS injected")
    }
}
