package com.euv

object AppConfig {
    const val REMOTE_URL = "https://ltpp.vip/euv"
    const val CACHE_DIR = "euv_web_cache"
    const val BACKGROUND_COLOR = "#ffffff"
    const val SPLASH_FADE_DURATION_MS = 300L
    const val SPLASH_MAX_WAIT_MS = 5000L
    const val IMMERSIVE_MODE = false
    const val TRANSPARENT_NAVIGATION_BAR = true
    const val ANTI_ALIASING = true
    const val MAX_FRAME_RATE_ENABLED = true
    val LOADING_HTML = """<!DOCTYPE html><html><head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0">
<title>Euv</title>
<style>body{margin:0;display:flex;align-items:center;justify-content:center;height:100vh;font-family:sans-serif;background:#ffffff;}
.loader{width:48px;height:48px}
</style>
</head><body><svg class="loader" viewBox="0 0 50 50" xmlns="http://www.w3.org/2000/svg"><circle cx="25" cy="25" r="20" fill="none" stroke="#e0e0e0" stroke-width="4"/><circle cx="25" cy="25" r="20" fill="none" stroke="#1677ff" stroke-width="4" stroke-linecap="round" stroke-dasharray="80 200" stroke-dashoffset="0"><animateTransform attributeName="transform" type="rotate" from="0 25 25" to="360 25 25" dur="1s" repeatCount="indefinite"/></circle></svg></body></html>"""
    val INDEX_HTML = """<!doctype html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover"
    />
    <title>Euv</title>
    <style>
      html,
      body {
        margin: 0;
        padding: 0;
        width: 100%;
        height: 100%;
        overflow: hidden;
        background: #ffffff;
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
"""
    const val KEEP_ALIVE_SERVICE = true
    const val NOTIFICATION_CHANNEL_ID = "euv_keep_alive"
    const val NOTIFICATION_CHANNEL_NAME = "Background Service"
    const val NOTIFICATION_TITLE = "EUV"
    const val NOTIFICATION_TEXT = "Running"
}
