(function () {
  if (window.__EUV_GPU_PROBE__) return;
  // Native compositor acceleration, WebGL, and WebGPU are distinct capabilities.
  // Run only on request; creating contexts is not part of normal page startup.
  window.__EUV_GPU_PROBE__ = async function () {
    var result = {
      secureContext: window.isSecureContext === true,
      webgl: { available: false, renderer: null, softwareRenderer: null },
      webgpu: { available: !!navigator.gpu, adapterAvailable: false }
    };
    var canvas = document.createElement('canvas');
    var context = null;
    try {
      context = canvas.getContext('webgl2', { powerPreference: 'high-performance' }) ||
        canvas.getContext('webgl', { powerPreference: 'high-performance' });
      if (context) {
        result.webgl.available = true;
        var debug = context.getExtension('WEBGL_debug_renderer_info');
        result.webgl.renderer = String(context.getParameter(debug ? debug.UNMASKED_RENDERER_WEBGL : context.RENDERER));
        if (/swiftshader|llvmpipe|softpipe|software rasterizer/i.test(result.webgl.renderer)) {
          result.webgl.softwareRenderer = true;
        }
      }
    } catch (error) {
      result.webgl.error = String(error);
    } finally {
      if (context) {
        var loss = context.getExtension('WEBGL_lose_context');
        if (loss) loss.loseContext();
      }
    }
    if (navigator.gpu) {
      var timeout;
      try {
        var adapter = await Promise.race([
          navigator.gpu.requestAdapter({ powerPreference: 'high-performance' }),
          new Promise(function (_, reject) {
            timeout = setTimeout(function () { reject(new Error('GPU adapter request timed out')); }, 2000);
          })
        ]);
        result.webgpu.adapterAvailable = !!adapter;
        if (adapter && adapter.info) {
          result.webgpu.adapter = {
            vendor: String(adapter.info.vendor || ''),
            architecture: String(adapter.info.architecture || ''),
            description: String(adapter.info.description || '')
          };
        }
      } catch (error) {
        result.webgpu.error = String(error);
      } finally {
        clearTimeout(timeout);
      }
    }
    return result;
  };
})();
