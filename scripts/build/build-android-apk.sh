#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../env/common.sh
source "${SCRIPT_DIR}/../env/common.sh"
load_local_env

if [ -z "${ANDROID_HOME:-}" ] || [ ! -x "${ANDROID_HOME}/platform-tools/adb" ]; then
  log_info "Android environment missing; running scripts/env/setup-android.sh..."
  "${REPO_ROOT}/scripts/env/setup-android.sh"
  load_local_env
fi

API_LEVEL="${ANDROID_API_LEVEL:-35}"
BUILD_TOOLS="${ANDROID_BUILD_TOOLS:-35.0.0}"
AVD_NAME="${ANDROID_AVD_NAME:-euv_android_${API_LEVEL}}"
ANDROID_JAR="${ANDROID_HOME}/platforms/android-${API_LEVEL}/android.jar"
BT_DIR="${ANDROID_HOME}/build-tools/${BUILD_TOOLS}"
AAPT2="${BT_DIR}/aapt2"
D8="${BT_DIR}/d8"
ZIPALIGN="${BT_DIR}/zipalign"
APKSIGNER="${BT_DIR}/apksigner"

BUILD_DIR="${REPO_ROOT}/target/mobile-hosts/android-host"
OUT_DIR="${REPO_ROOT}/dist/android"
APK_OUT="${OUT_DIR}/EuvShowcase.apk"
mkdir -p "${BUILD_DIR}/rust/src" "${BUILD_DIR}/java/vip/ltpp/euv/showcase" "${BUILD_DIR}/stage/lib/arm64-v8a" "${OUT_DIR}"

cat > "${BUILD_DIR}/rust/Cargo.toml" <<EOF
[workspace]

[package]
name = "euv-android-host"
version = "0.38.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
euv = { path = "${REPO_ROOT}" }
euv-example = { path = "${REPO_ROOT}/example" }
EOF

cat > "${BUILD_DIR}/rust/src/lib.rs" <<'EOF'
use std::ffi::c_void;
use euv::{
    App, NativeSystemFont, PointerEvent, RenderMode, SafeAreaInsets, WindowCommand, WindowEvent,
    WindowId,
};
use euv_example::{ShowcaseState, create_showcase_app};

pub type JniEnv = *mut *const JniNativeInterface;
pub type JObject = *mut c_void;
pub type JIntArray = *mut c_void;

#[repr(C)]
pub struct JniNativeInterface {
    reserved: [*const c_void; 211],
    set_int_array_region: unsafe extern "C" fn(JniEnv, JIntArray, i32, i32, *const i32),
}

pub struct EuvAndroidHandle {
    app: App,
    _state: ShowcaseState,
    wid: WindowId,
    argb: Vec<i32>,
    width: u32,
    height: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeCreate(
    _env: JniEnv,
    _cls: JObject,
    width: f32,
    height: f32,
    scale: f32,
) -> i64 {
    let Ok((mut app, state)) = create_showcase_app(true, RenderMode::Software) else {
        return 0;
    };
    app.get_mut_renderer()
        .get_mut_software()
        .set_font_provider(NativeSystemFont::system());
    let wid: WindowId = app.get_window_state().get_id();
    let w: u32 = width.max(320.0).round() as u32;
    let h: u32 = height.max(480.0).round() as u32;
    let s: f32 = scale.clamp(1.0, 3.0);
    app.queue_window_command(WindowCommand::SetSize(wid, w, h));
    app.dispatch_window_event(WindowEvent::Resized(wid, w, h, s));
    let _ = app.step_game_frame(1.0 / 60.0);
    let handle = Box::new(EuvAndroidHandle {
        app,
        _state: state,
        wid,
        argb: Vec::new(),
        width: 0,
        height: 0,
    });
    Box::into_raw(handle) as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeResize(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
    width: f32,
    height: f32,
    scale: f32,
) {
    if ptr == 0 {
        return;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let w: u32 = width.max(320.0).round() as u32;
    let ht: u32 = height.max(480.0).round() as u32;
    let s: f32 = scale.clamp(1.0, 3.0);
    h.app.queue_window_command(WindowCommand::SetSize(h.wid, w, ht));
    h.app
        .dispatch_window_event(WindowEvent::Resized(h.wid, w, ht, s));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeSetSafeArea(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
) {
    if ptr == 0 {
        return;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::SafeAreaChanged(
        h.wid,
        SafeAreaInsets::new(top.max(0.0), right.max(0.0), bottom.max(0.0), left.max(0.0)),
    ));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeTouchDown(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
    pointer_id: i32,
    x: f32,
    y: f32,
) {
    if ptr == 0 {
        return;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::PointerDown(
        h.wid,
        PointerEvent::touch((pointer_id as u64).saturating_add(1), x, y),
    ));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeTouchMove(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
    pointer_id: i32,
    x: f32,
    y: f32,
) {
    if ptr == 0 {
        return;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::PointerMove(
        h.wid,
        PointerEvent::touch((pointer_id as u64).saturating_add(1), x, y),
    ));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeTouchUp(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
    pointer_id: i32,
    x: f32,
    y: f32,
) {
    if ptr == 0 {
        return;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::PointerUp(
        h.wid,
        PointerEvent::touch((pointer_id as u64).saturating_add(1), x, y),
    ));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeStepFrame(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    dt: f32,
    out_pixels: JIntArray,
    max_len: i32,
) -> i32 {
    if ptr == 0 || env.is_null() || out_pixels.is_null() {
        return 0;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let Ok(stats) = h.app.step_game_frame(dt.clamp(1.0 / 240.0, 0.05)) else {
        return 0;
    };
    let changed: bool = stats.get_pixels_shaded() > 0 || h.argb.is_empty();
    if changed && let Some(fb) = h.app.get_renderer().framebuffer() {
        let w: u32 = fb.get_width();
        let ht: u32 = fb.get_height();
        let src: &[u8] = fb.get_pixels().as_slice();
        let pixel_count: usize = (w as usize) * (ht as usize);
        h.width = w;
        h.height = ht;
        h.argb.resize(pixel_count, 0);
        for (idx, chunk) in src.chunks_exact(4).take(pixel_count).enumerate() {
            let r: u32 = chunk[0] as u32;
            let g: u32 = chunk[1] as u32;
            let b: u32 = chunk[2] as u32;
            let a: u32 = chunk[3] as u32;
            h.argb[idx] = ((a << 24) | (r << 16) | (g << 8) | b) as i32;
        }
        let copy_len: i32 = (pixel_count as i32).min(max_len);
        if copy_len > 0 {
            unsafe {
                let table: &JniNativeInterface = &*(*env);
                (table.set_int_array_region)(env, out_pixels, 0, copy_len, h.argb.as_ptr());
            }
        }
    }
    if changed { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeGetWidth(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
) -> i32 {
    if ptr == 0 {
        return 0;
    }
    let h: &EuvAndroidHandle = unsafe { &*(ptr as *const EuvAndroidHandle) };
    h.width as i32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeGetHeight(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
) -> i32 {
    if ptr == 0 {
        return 0;
    }
    let h: &EuvAndroidHandle = unsafe { &*(ptr as *const EuvAndroidHandle) };
    h.height as i32
}
EOF

cat > "${BUILD_DIR}/java/vip/ltpp/euv/showcase/EuvNativeBridge.java" <<'EOF'
package vip.ltpp.euv.showcase;

public final class EuvNativeBridge {
    static {
        System.loadLibrary("euv_android_host");
    }

    public static native long nativeCreate(float width, float height, float scale);
    public static native void nativeResize(long handle, float width, float height, float scale);
    public static native void nativeSetSafeArea(long handle, float top, float right, float bottom, float left);
    public static native void nativeTouchDown(long handle, int pointerId, float x, float y);
    public static native void nativeTouchMove(long handle, int pointerId, float x, float y);
    public static native void nativeTouchUp(long handle, int pointerId, float x, float y);
    public static native int nativeStepFrame(long handle, float dt, int[] outPixels, int maxLen);
    public static native int nativeGetWidth(long handle);
    public static native int nativeGetHeight(long handle);
}
EOF

cat > "${BUILD_DIR}/java/vip/ltpp/euv/showcase/EuvActivity.java" <<'EOF'
package vip.ltpp.euv.showcase;

import android.app.Activity;
import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Insets;
import android.graphics.Rect;
import android.os.Bundle;
import android.view.Choreographer;
import android.view.MotionEvent;
import android.view.View;
import android.view.Window;
import android.view.WindowInsets;
import android.view.WindowInsetsController;
import android.view.WindowManager;

public final class EuvActivity extends Activity {
    private EuvCanvasView canvasView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        Window win = getWindow();
        win.setDecorFitsSystemWindows(false);
        win.setStatusBarColor(Color.TRANSPARENT);
        win.setNavigationBarColor(Color.TRANSPARENT);
        WindowManager.LayoutParams lp = win.getAttributes();
        lp.layoutInDisplayCutoutMode = WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES;
        win.setAttributes(lp);
        canvasView = new EuvCanvasView(this);
        setContentView(canvasView);
        canvasView.post(() -> {
            WindowInsetsController ctrl = win.getInsetsController();
            if (ctrl != null) {
                ctrl.setSystemBarsAppearance(
                    WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS | WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS,
                    WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS | WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS
                );
            }
        });
    }

    @Override
    protected void onResume() {
        super.onResume();
        if (canvasView != null) {
            canvasView.startLoop();
        }
    }

    @Override
    protected void onPause() {
        super.onPause();
        if (canvasView != null) {
            canvasView.stopLoop();
        }
    }

    private static final class EuvCanvasView extends View implements Choreographer.FrameCallback {
        private long handle = 0;
        private float displayDensity = 2.0f;
        private float renderScale = 2.0f;
        private int[] pixelBuffer = new int[1];
        private Bitmap bitmap = null;
        private long lastNanos = 0;
        private boolean running = false;
        private final Rect dstRect = new Rect();

        EuvCanvasView(Context ctx) {
            super(ctx);
            displayDensity = Math.max(1.0f, ctx.getResources().getDisplayMetrics().density);
            renderScale = Math.max(1.0f, Math.min(3.0f, displayDensity));
            setOnApplyWindowInsetsListener((v, insets) -> {
                Insets bars = insets.getInsets(WindowInsets.Type.systemBars() | WindowInsets.Type.displayCutout());
                if (handle != 0) {
                    EuvNativeBridge.nativeSetSafeArea(
                        handle,
                        bars.top / displayDensity,
                        bars.right / displayDensity,
                        bars.bottom / displayDensity,
                        bars.left / displayDensity
                    );
                }
                return insets;
            });
        }

        private void ensurePixelBuffer(int needed) {
            if (pixelBuffer.length < needed) {
                pixelBuffer = new int[Math.max(1, needed)];
            }
        }

        private void ensureBitmap(int bw, int bh) {
            if (bitmap == null || bitmap.getWidth() != bw || bitmap.getHeight() != bh) {
                bitmap = Bitmap.createBitmap(Math.max(1, bw), Math.max(1, bh), Bitmap.Config.ARGB_8888);
            }
        }

        @Override
        protected void onSizeChanged(int w, int h, int oldw, int oldh) {
            super.onSizeChanged(w, h, oldw, oldh);
            if (w <= 0 || h <= 0) return;
            float logicalW = w / displayDensity;
            float logicalH = h / displayDensity;
            if (handle == 0) {
                handle = EuvNativeBridge.nativeCreate(logicalW, logicalH, renderScale);
                requestApplyInsets();
            } else {
                EuvNativeBridge.nativeResize(handle, logicalW, logicalH, renderScale);
            }
            int physW = Math.max(1, Math.round(logicalW * renderScale) + 8);
            int physH = Math.max(1, Math.round(logicalH * renderScale) + 8);
            ensurePixelBuffer(physW * physH);
            stepAndBlit(1.0f / 60.0f);
        }

        void startLoop() {
            if (!running) {
                running = true;
                lastNanos = 0;
                Choreographer.getInstance().postFrameCallback(this);
            }
        }

        void stopLoop() {
            running = false;
            Choreographer.getInstance().removeFrameCallback(this);
        }

        @Override
        public void doFrame(long frameTimeNanos) {
            if (!running) return;
            float dt = (lastNanos > 0) ? ((frameTimeNanos - lastNanos) / 1_000_000_000.0f) : (1.0f / 60.0f);
            lastNanos = frameTimeNanos;
            stepAndBlit(dt);
            Choreographer.getInstance().postFrameCallback(this);
        }

        private void stepAndBlit(float dt) {
            if (handle == 0 || getWidth() <= 0 || getHeight() <= 0) return;
            int physW = Math.max(1, Math.round((getWidth() / displayDensity) * renderScale) + 8);
            int physH = Math.max(1, Math.round((getHeight() / displayDensity) * renderScale) + 8);
            ensurePixelBuffer(physW * physH);
            int changed = EuvNativeBridge.nativeStepFrame(handle, dt, pixelBuffer, pixelBuffer.length);
            if (changed != 0) {
                int bw = EuvNativeBridge.nativeGetWidth(handle);
                int bh = EuvNativeBridge.nativeGetHeight(handle);
                if (bw > 0 && bh > 0 && bw * bh <= pixelBuffer.length) {
                    ensureBitmap(bw, bh);
                    bitmap.setPixels(pixelBuffer, 0, bw, 0, 0, bw, bh);
                    invalidate();
                }
            }
        }

        @Override
        protected void onDraw(Canvas canvas) {
            super.onDraw(canvas);
            if (bitmap != null) {
                dstRect.set(0, 0, getWidth(), getHeight());
                canvas.drawBitmap(bitmap, null, dstRect, null);
            }
        }

        @Override
        public boolean onTouchEvent(MotionEvent event) {
            if (handle == 0) return false;
            int action = event.getActionMasked();
            int actionIdx = event.getActionIndex();
            switch (action) {
                case MotionEvent.ACTION_DOWN:
                case MotionEvent.ACTION_POINTER_DOWN: {
                    int pid = event.getPointerId(actionIdx);
                    float x = event.getX(actionIdx) / displayDensity;
                    float y = event.getY(actionIdx) / displayDensity;
                    EuvNativeBridge.nativeTouchDown(handle, pid, x, y);
                    stepAndBlit(1.0f / 60.0f);
                    return true;
                }
                case MotionEvent.ACTION_MOVE: {
                    int count = event.getPointerCount();
                    for (int i = 0; i < count; i++) {
                        int pid = event.getPointerId(i);
                        float x = event.getX(i) / displayDensity;
                        float y = event.getY(i) / displayDensity;
                        EuvNativeBridge.nativeTouchMove(handle, pid, x, y);
                    }
                    return true;
                }
                case MotionEvent.ACTION_UP:
                case MotionEvent.ACTION_POINTER_UP:
                case MotionEvent.ACTION_CANCEL: {
                    int pid = event.getPointerId(actionIdx);
                    float x = event.getX(actionIdx) / displayDensity;
                    float y = event.getY(actionIdx) / displayDensity;
                    EuvNativeBridge.nativeTouchUp(handle, pid, x, y);
                    stepAndBlit(1.0f / 60.0f);
                    return true;
                }
                default:
                    return super.onTouchEvent(event);
            }
        }
    }
}
EOF

cat > "${BUILD_DIR}/AndroidManifest.xml" <<'EOF'
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="vip.ltpp.euv.showcase"
    android:versionCode="1"
    android:versionName="0.38.0">
    <uses-sdk android:minSdkVersion="30" android:targetSdkVersion="35" />
    <application
        android:label="euv Showcase"
        android:icon="@mipmap/ic_launcher"
        android:roundIcon="@mipmap/ic_launcher"
        android:theme="@android:style/Theme.DeviceDefault.Light.NoActionBar"
        android:hardwareAccelerated="true">
        <activity
            android:name=".EuvActivity"
            android:exported="true"
            android:configChanges="orientation|screenSize|screenLayout|smallestScreenSize|keyboardHidden">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
EOF

log_info "Building Release aarch64-linux-android native shared library..."
(
  cd "${BUILD_DIR}/rust"
  cargo build --release --target aarch64-linux-android
)
SO_PATH="${BUILD_DIR}/rust/target/aarch64-linux-android/release/libeuv_android_host.so"
ANDROID_SCHEMA="$(strings "${SO_PATH}" | grep -c "euv.ai.schema.v1" || true)"
log_info "Built ${SO_PATH} ($(du -h "${SO_PATH}" | cut -f1)), release ai-schema matches=${ANDROID_SCHEMA}"
[ "${ANDROID_SCHEMA}" -eq 0 ]

log_info "Compiling Android Java host & DEX..."
rm -rf "${BUILD_DIR}/classes" "${BUILD_DIR}/dex"
mkdir -p "${BUILD_DIR}/classes" "${BUILD_DIR}/dex"
javac -source 11 -target 11 -classpath "${ANDROID_JAR}" -d "${BUILD_DIR}/classes" \
  "${BUILD_DIR}/java/vip/ltpp/euv/showcase/EuvNativeBridge.java" \
  "${BUILD_DIR}/java/vip/ltpp/euv/showcase/EuvActivity.java"
"${D8}" --min-api 30 --lib "${ANDROID_JAR}" --output "${BUILD_DIR}/dex" \
  "${BUILD_DIR}/classes/vip/ltpp/euv/showcase/"*.class

log_info "Generating app icon resources and packaging EuvShowcase.apk..."
RES_DIR="${BUILD_DIR}/res"
RES_ZIP="${BUILD_DIR}/res.zip"
rm -rf "${RES_DIR}" "${RES_ZIP}"
mkdir -p "${RES_DIR}/mipmap-xxhdpi" "${RES_DIR}/mipmap-xxxhdpi"
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${RES_DIR}/mipmap-xxhdpi/ic_launcher.png" 144 "${EUV_APP_ICON:-}"
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${RES_DIR}/mipmap-xxxhdpi/ic_launcher.png" 192 "${EUV_APP_ICON:-}"
"${AAPT2}" compile --dir "${RES_DIR}" -o "${RES_ZIP}"

UNALIGNED_APK="${BUILD_DIR}/unaligned.apk"
ALIGNED_APK="${BUILD_DIR}/aligned.apk"
rm -f "${UNALIGNED_APK}" "${ALIGNED_APK}" "${APK_OUT}"
"${AAPT2}" link \
  -I "${ANDROID_JAR}" \
  --manifest "${BUILD_DIR}/AndroidManifest.xml" \
  -R "${RES_ZIP}" \
  -o "${UNALIGNED_APK}"

rm -rf "${BUILD_DIR}/stage" && mkdir -p "${BUILD_DIR}/stage/lib/arm64-v8a"
cp "${BUILD_DIR}/dex/classes.dex" "${BUILD_DIR}/stage/classes.dex"
cp "${SO_PATH}" "${BUILD_DIR}/stage/lib/arm64-v8a/libeuv_android_host.so"
(
  cd "${BUILD_DIR}/stage"
  zip -q -0 -r "${UNALIGNED_APK}" classes.dex lib/arm64-v8a/libeuv_android_host.so
)
"${ZIPALIGN}" -f -p 4 "${UNALIGNED_APK}" "${ALIGNED_APK}"

KEYSTORE="${HOME}/.android/debug.keystore"
if [ ! -f "${KEYSTORE}" ]; then
  mkdir -p "${HOME}/.android"
  keytool -genkeypair -v -keystore "${KEYSTORE}" -storepass android -alias androiddebugkey \
    -keypass android -keyalg RSA -keysize 2048 -validity 10000 \
    -dname "CN=Android Debug,O=Android,C=US" >/dev/null 2>&1
fi
"${APKSIGNER}" sign --ks "${KEYSTORE}" --ks-pass pass:android --key-pass pass:android --out "${APK_OUT}" "${ALIGNED_APK}"
"${APKSIGNER}" verify "${APK_OUT}"
log_info "Signed APK ready: ${APK_OUT} ($(du -h "${APK_OUT}" | cut -f1))"

if [ "${1:-}" = "--install" ]; then
  adb start-server >/dev/null 2>&1 || true
  DEV_COUNT="$(adb devices | awk 'NR>1 && $2=="device" {print $1}' | wc -l | tr -d ' ')"
  if [ "${DEV_COUNT}" -eq 0 ]; then
    log_info "No online ADB device detected; booting AVD '${AVD_NAME}'..."
    nohup "${ANDROID_HOME}/emulator/emulator" -avd "${AVD_NAME}" -no-boot-anim -gpu auto >/tmp/euv_android_emulator.log 2>&1 &
    adb wait-for-device
    for _ in $(seq 1 90); do
      if [ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ]; then
        break
      fi
      sleep 2
    done
    sleep 3
  fi
  log_info "Installing ${APK_OUT} via adb..."
  adb devices -l
  ONLINE_SERIALS="$(adb devices | awk 'NR>1 && $2=="device" {print $1}')"
  for SERIAL in ${ONLINE_SERIALS}; do
    log_info "Deploying to Android target ${SERIAL}..."
    adb -s "${SERIAL}" install -r "${APK_OUT}"
    adb -s "${SERIAL}" shell am start -n vip.ltpp.euv.showcase/.EuvActivity
  done
  log_info "Launched vip.ltpp.euv.showcase/.EuvActivity on all online Android targets."
fi
