#!/usr/bin/env node
/**
 * apply-config.js
 *
 * Reads app.config.json and injects configuration into platform-specific files:
 * - tauri.conf.json
 * - dist/index.html
 * - Android strings.xml
 * - Android AppConfig.kt
 *
 * Usage: node scripts/apply-config.js
 */

const fs = require('fs');
const path = require('path');

const ROOT = path.resolve(__dirname, '..');
const CONFIG_PATH = path.join(ROOT, 'app.config.json');

function loadConfig() {
  if (!fs.existsSync(CONFIG_PATH)) {
    console.error('[ERROR] app.config.json not found at:', CONFIG_PATH);
    process.exit(1);
  }
  return JSON.parse(fs.readFileSync(CONFIG_PATH, 'utf-8'));
}

/** Preserve generated-file timestamps unless their contents change. */
function writeIfChanged(file, contents) {
  const bytes = Buffer.isBuffer(contents) ? contents : Buffer.from(contents);
  try {
    if (fs.readFileSync(file).equals(bytes)) return false;
  } catch (error) {
    if (error.code !== 'ENOENT') throw error;
  }
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, bytes);
  return true;
}

function writeTauriConf(config) {
  const tauriConfPath = path.join(ROOT, 'src-tauri', 'tauri.conf.json');
  const tauriConf = {
    $schema: 'https://schema.tauri.app/config/2',
    productName: config.app.name,
    version: config.app.version,
    identifier: config.app.identifier,
    build: {
      frontendDist: '../dist',
      beforeDevCommand: '',
      beforeBuildCommand:
        'node scripts/prefetch-cache.js && node scripts/apply-config.js',
    },
    app: {
      withGlobalTauri: true,
      windows: [
        {
          title: config.ui.window.title,
          width: config.ui.window.width,
          height: config.ui.window.height,
          resizable: config.ui.window.resizable,
          fullscreen: config.ui.window.fullscreen,
          useHttpsScheme: true,
        },
      ],
      security: {
        csp: null,
      },
    },
    bundle: {
      active: true,
      targets: 'all',
      icon: [
        'icons/32x32.png',
        'icons/128x128.png',
        'icons/128x128@2x.png',
        'icons/icon.ico',
      ],
      android: {
        debugApplicationIdSuffix: '.debug',
      },
    },
  };
  writeIfChanged(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');
  console.log('[OK] tauri.conf.json updated');
}

/**
 * Generate the default loading HTML (used as fallback when offline and no cache).
 * Can be overridden by setting ui.loadingHtml in app.config.json.
 */
function getLoadingHtml(config) {
  if (config.ui.loadingHtml) {
    return config.ui.loadingHtml;
  }
  const bg = config.ui.backgroundColor;
  const spinnerColor = config.ui.loadingSpinnerColor || '#1677ff';
  const trackColor = config.ui.loadingSpinnerTrackColor || '#e0e0e0';
  return `<!DOCTYPE html><html><head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0">
<title>${config.app.name}</title>
<style>body{margin:0;display:flex;align-items:center;justify-content:center;height:100vh;font-family:sans-serif;background:${bg};}
.loader{width:48px;height:48px}
</style>
</head><body><svg class="loader" viewBox="0 0 50 50" xmlns="http://www.w3.org/2000/svg"><circle cx="25" cy="25" r="20" fill="none" stroke="${trackColor}" stroke-width="4"/><circle cx="25" cy="25" r="20" fill="none" stroke="${spinnerColor}" stroke-width="4" stroke-linecap="round" stroke-dasharray="80 200" stroke-dashoffset="0"><animateTransform attributeName="transform" type="rotate" from="0 25 25" to="360 25 25" dur="1s" repeatCount="indefinite"/></circle></svg></body></html>`;
}

/**
 * Generate the index.html that Tauri loads as the frontend entry point.
 *
 * This is a minimal page — on Android the native Kotlin splash overlay handles
 * logo display, so we only need the JS that waits for cache readiness and then
 * navigates to the cached app served via the custom protocol.
 *
 * Can be overridden by setting ui.indexHtml in app.config.json.
 */
function getIndexHtml(config) {
  if (config.ui.indexHtml) {
    return config.ui.indexHtml;
  }
  const bg = config.ui.backgroundColor;

  return `<!doctype html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover"
    />
    <title>${config.app.name}</title>
    <style>
      html,
      body {
        margin: 0;
        padding: 0;
        width: 100%;
        height: 100%;
        overflow: hidden;
        background: ${bg};
      }
    </style>
  </head>
  <body>
    <script>
      (function () {
        function getSchemeUrl(path) {
          var ua = navigator.userAgent || navigator.platform || '';
          return /Win|Android/.test(ua)
            ? 'https://euv.localhost/' + path
            : 'euv://localhost/' + path;
        }
        var stopped = false;
        var pending = false;
        var timer = null;
        var bridgeDelay = 16;
        var retryDelay = 250;
        function schedule(delay) {
          if (stopped) return;
          timer = setTimeout(function () {
            timer = null;
            waitForCache();
          }, delay);
        }
        function retry() {
          pending = false;
          schedule(retryDelay);
          retryDelay = Math.min(retryDelay * 2, 5000);
        }
        function waitForCache() {
          if (stopped || pending) return;
          var internals = window.__TAURI_INTERNALS__;
          if (!internals || typeof internals.invoke !== 'function') {
            schedule(bridgeDelay);
            bridgeDelay = Math.min(bridgeDelay * 2, 250);
            return;
          }
          pending = true;
          try {
            internals.invoke('wait_for_cached_resource').then(function (result) {
              pending = false;
              if (stopped) return;
              if (!result || !result.from_cache) {
                retry();
                return;
              }
              stopped = true;
              window.location.replace(getSchemeUrl('index.html'));
            }, retry);
          } catch (error) {
            retry();
          }
        }
        window.addEventListener('pagehide', function () {
          stopped = true;
          if (timer !== null) clearTimeout(timer);
        });
        waitForCache();
      })();
    </script>
  </body>
</html>
`;
}

function writeDistIndex(config) {
  const indexPath = path.join(ROOT, 'dist', 'index.html');
  const html = getIndexHtml(config);
  fs.mkdirSync(path.dirname(indexPath), { recursive: true });
  writeIfChanged(indexPath, html);
  console.log('[OK] dist/index.html updated');
}

function writeAndroidStrings(config) {
  const stringsPath = path.join(
    ROOT,
    'src-tauri',
    'gen',
    'android',
    'app',
    'src',
    'main',
    'res',
    'values',
    'strings.xml',
  );
  const xml = `<resources>
    <string name="app_name">${config.app.name}</string>
    <string name="main_activity_title">${config.app.name}</string>
</resources>
`;
  writeIfChanged(stringsPath, xml);
  console.log('[OK] Android strings.xml updated');
}

function writeAndroidConfig(config) {
  const configKtPath = path.join(
    ROOT,
    'src-tauri',
    'gen',
    'android',
    'app',
    'src',
    'main',
    'java',
    'com',
    'euv',
    'AppConfig.kt',
  );

  // Generate loading HTML for Android (escaped for Kotlin raw string)
  const loadingHtml = getLoadingHtml(config);
  // Escape $ signs for Kotlin raw strings (triple-quoted)
  const loadingHtmlKt = loadingHtml.replace(/\$/g, '\${"\$"}');

  // Generate index HTML for Android
  const indexHtml = getIndexHtml(config);
  const indexHtmlKt = indexHtml.replace(/\$/g, '\${"\$"}');

  const kt = `package com.euv

object AppConfig {
    const val REMOTE_URL = "${config.remote.url}"
    const val CACHE_DIR = "${config.cache.directory}"
    const val BACKGROUND_COLOR = "${config.ui.backgroundColor}"
    const val SPLASH_FADE_DURATION_MS = ${config.ui.splashFadeDurationMs}L
    const val SPLASH_MAX_WAIT_MS = ${config.ui.splashMaxWaitMs}L
    const val IMMERSIVE_MODE = ${config.ui.immersiveMode}
    const val TRANSPARENT_NAVIGATION_BAR = ${config.ui.transparentNavigationBar ?? false}
    const val ANTI_ALIASING = ${config.ui.antiAliasing ?? false}
    const val MAX_FRAME_RATE_ENABLED = ${config.ui.maxFrameRateEnabled ?? false}
    val LOADING_HTML = """${loadingHtmlKt}"""
    val INDEX_HTML = """${indexHtmlKt}"""
    const val KEEP_ALIVE_SERVICE = ${config.android.keepAliveService}
    const val NOTIFICATION_CHANNEL_ID = "${config.android.notification.channelId}"
    const val NOTIFICATION_CHANNEL_NAME = "${config.android.notification.channelName}"
    const val NOTIFICATION_TITLE = "${config.android.notification.title}"
    const val NOTIFICATION_TEXT = "${config.android.notification.text}"
}
`;
  fs.mkdirSync(path.dirname(configKtPath), { recursive: true });
  writeIfChanged(configKtPath, kt);
  console.log('[OK] Android AppConfig.kt generated');
}

function writeAndroidBuildGradle(config) {
  const gradlePath = path.join(
    ROOT,
    'src-tauri',
    'gen',
    'android',
    'app',
    'build.gradle.kts',
  );
  if (!fs.existsSync(gradlePath)) {
    return;
  }
  let gradle = fs.readFileSync(gradlePath, 'utf-8');
  const targetSdk = config.android.targetSdk || 36;
  const compileSdk = config.android.compileSdk || 36;
  const minSdk = config.android.minSdk || 24;
  gradle = gradle.replace(/compileSdk\s*=\s*\d+/, `compileSdk = ${compileSdk}`);
  gradle = gradle.replace(/minSdk\s*=\s*\d+/, `minSdk = ${minSdk}`);
  gradle = gradle.replace(/targetSdk\s*=\s*\d+/, `targetSdk = ${targetSdk}`);
  writeIfChanged(gradlePath, gradle);
  console.log(
    `[OK] Android build.gradle.kts SDK versions updated (compileSdk=${compileSdk}, minSdk=${minSdk}, targetSdk=${targetSdk})`,
  );
}

function main() {
  console.log('[apply-config] Reading app.config.json...');
  const config = loadConfig();
  console.log(`[apply-config] App: ${config.app.name} v${config.app.version}`);

  writeTauriConf(config);
  writeDistIndex(config);
  writeAndroidStrings(config);
  writeAndroidConfig(config);
  writeAndroidBuildGradle(config);

  console.log('[apply-config] Done! All platform configs updated.');
}

module.exports = { getIndexHtml, writeIfChanged };

if (require.main === module) main();
