'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { test } = require('node:test');

const root = path.resolve(__dirname, '../..');
const activityPath = path.join(
  root,
  'src-tauri/gen/android/app/src/main/java/com/euv/MainActivity.kt',
);
const source = fs.readFileSync(activityPath, 'utf8');

test('immersive mode hides the system bars with transient swipe behavior', () => {
  assert.match(source, /controller\.hide\(WindowInsetsCompat\.Type\.systemBars\(\)\)/);
  assert.match(
    source,
    /WindowInsetsControllerCompat\.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE/,
  );
  // Legacy path keeps the bars hidden after a transient swipe reveal.
  assert.match(source, /View\.SYSTEM_UI_FLAG_IMMERSIVE_STICKY/);
  assert.match(source, /View\.SYSTEM_UI_FLAG_HIDE_NAVIGATION/);
  assert.match(source, /View\.SYSTEM_UI_FLAG_FULLSCREEN/);
});

test('immersive state is re-applied on focus and resume', () => {
  assert.match(source, /override fun onWindowFocusChanged\(hasFocus: Boolean\)/);
  assert.match(source, /override fun onResume\(\)/);
  // The focus path and the resume path both re-run the immersive setup.
  const focus = source.slice(
    source.indexOf('override fun onWindowFocusChanged'),
    source.indexOf('override fun onResume()'),
  );
  assert.match(focus, /enableImmersiveMode\(\)/);
  assert.match(focus, /setMaxFrameRate\(\)/);
  const resume = source.slice(
    source.indexOf('override fun onResume()'),
    source.indexOf('override fun onConfigurationChanged'),
  );
  assert.match(resume, /enableImmersiveMode\(\)/);
  assert.match(resume, /setMaxFrameRate\(\)/);
});

test('display changes are observed with a symmetrically unregistered listener', () => {
  assert.match(source, /registerDisplayListener\(listener, mainHandler\)/);
  assert.match(source, /unregisterDisplayListener\(listener\)/);
  assert.match(source, /override fun onDisplayAdded\(displayId: Int\)/);
  assert.match(source, /override fun onDisplayChanged\(displayId: Int\)/);
  assert.match(source, /override fun onDisplayRemoved\(displayId: Int\)/);
  // Registration is guarded so it happens at most once.
  assert.match(source, /if \(displayListener != null\) return/);
  // Teardown removes the listener before the activity goes away.
  const destroy = source.slice(
    source.indexOf('override fun onDestroy()'),
    source.indexOf('override fun onDestroy()') + 400,
  );
  assert.match(destroy, /unregisterDisplayListener\(\)/);
  assert.match(destroy, /mainHandler\.removeCallbacksAndMessages\(null\)/);
});

test('refresh rate is requested from the current display without pinning a mode', () => {
  // The requested rate is derived from the display's own supported modes.
  assert.match(source, /display\.supportedModes/);
  assert.match(source, /sameSizeModes\.maxOfOrNull \{ it\.refreshRate \} \?: currentMode\.refreshRate/);
  assert.match(source, /it\.physicalWidth == currentMode\.physicalWidth/);
  assert.match(source, /it\.physicalHeight == currentMode\.physicalHeight/);
  assert.doesNotMatch(source, /else modes\.asList\(\)/);
  // preferredDisplayModeId would also pin the resolution, so it must not be
  // used to request a refresh rate.
  assert.doesNotMatch(source, /params\.preferredDisplayModeId = maxRefreshMode/);
  assert.match(source, /params\.preferredRefreshRate = refreshRate/);
  assert.match(source, /params\.preferredDisplayModeId = 0/);
});

test('refresh rate handling uses public APIs only', () => {
  // The previous implementation reached into SurfaceControl via reflection.
  assert.doesNotMatch(source, /SurfaceControl/);
  assert.doesNotMatch(source, /rootSurfaceControl/);
  assert.doesNotMatch(source, /FRAME_RATE_COMPATIBILITY_FIXED_SOURCE/);
  assert.doesNotMatch(source, /CHANGE_FRAME_RATE_ONLY_IF_SEAMLESS/);
  // The modern view-level vote is guarded by an API check.
  assert.match(source, /Build\.VERSION_CODES\.VANILLA_ICE_CREAM/);
  assert.match(source, /setRequestedFrameRate\(refreshRate\)/);
});

test('no refresh rate is hardcoded and no artificial frame timer is introduced', () => {
  const refreshRateNumbers = source.match(
    /\b(?:60|90|120|144)\s*(?:\.0f)?\s*(?:Hz|fps)?/gi,
  ) || [];
  for (const literal of refreshRateNumbers) {
    assert.fail(`hardcoded refresh rate literal found: ${literal}`);
  }
  // Browser vsync is preserved: no frame-rate driving timer is added.
  assert.doesNotMatch(source, /postDelayed\([^)]*\b(?:16|8)\b/);
  // The existing splash poll timer is untouched.
  assert.match(source, /handler\.postDelayed\(this, 32\)/);
});

test('offline caching, splash and safe-area features are preserved', () => {
  assert.match(source, /private fun addSplashOverlay\(\)/);
  assert.match(source, /fun removeSplash\(\)/);
  assert.match(source, /setOnApplyWindowInsetsListener/);
  assert.match(source, /--euv-keyboard-height/);
  assert.match(source, /startKeepAliveServiceSafely\(\)/);
  assert.match(source, /injectBridgeScript\(webView\)/);
  assert.match(source, /LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES/);
});
