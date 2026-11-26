---
title: Native GPU Drivers, System Fonts & UI Catalog
order: 3
section: guide
tagline: Physical Metal/D3D12/Vulkan/WebGL2 GPU drivers, CoreText/GDI/FreeType Unicode & CJK shaping, and 59 euv-ui components.
---

## Physical Native GPU Drivers

`NativeGpuDriver::probe` binds directly to the host OS graphics stack without external wrapper bloat:

- **macOS, iOS, and iPadOS (`Metal.framework`)**: Initializes `MTLCreateSystemDefaultDevice`, compiles `MSL_UI_SHADER` (`euv_ui_vertex` and `euv_ui_fragment`) at runtime via `newLibraryWithSource`, uploads instanced `GpuQuadInstance` vertex buffers (`MTLBuffer`), and commits `MTLCommandBuffer` frames.
- **Windows (`d3d12.dll` + `d3dcompiler_47.dll`)**: Runtime `LoadLibraryA` / `GetProcAddress` probing for Direct3D 12 and HLSL compilation with automatic Win32 GDI fallback.
- **Linux and Android (`libvulkan.so.1` / `libvulkan.so`)**: Runtime `dlopen` Vulkan loader probing with automatic software fallback on headless containers.
- **Web (`WebGl2RenderingContext`)**: Compiles and links `GLSL_ES3_UI_VERTEX_SHADER` + `GLSL_ES3_UI_FRAGMENT_SHADER` into a live `WebGlProgram`.

## System Fonts & CJK Shaping

`NativeSystemFont::system()` dispatches glyph rasterization to native OS text engines:

- **Apple CoreText + CoreGraphics**: Automatic `CTFontCreateForString` font cascade supporting Latin (`SF Pro`) and CJK (`PingFang SC`) anti-aliased glyphs.
- **Windows Win32 GDI**: `GetGlyphOutlineW` (`GGO_GRAY8_BITMAP`) smooth coverage extraction.
- **Linux / Android FreeType & Web Canvas**: Runtime system font discovery with deterministic Unicode/CJK stroke fallback.
- **Packed GPU Glyph Atlas**: `GlyphAtlas::pack_rgba_atlas()` shelf-packs cached glyphs into a single RGBA8 texture ready for GPU upload.

## Complete `euv-ui` Component Library (59 Components)

- **Layout & Surface**: `EuvAppShell`, `EuvPageContainer`, `EuvPanel`, `EuvSpace`, `EuvGrid`, `EuvScrollArea`, `EuvSplitter`, `EuvNavbar`, `EuvFooter`, `EuvDivider`, `EuvButtonGroup`, `EuvToggle`.
- **Display & Data**: `EuvHeader`, `EuvCard`, `EuvStatCard`, `EuvBadge`, `EuvTag`, `EuvAvatar`, `EuvIcon`, `EuvKbd`, `EuvTypography`, `EuvEmpty`, `EuvSkeleton`, `EuvSpinner`, `EuvProgress`, `EuvCodeBlock`, `EuvTimeline`, `EuvList`, `EuvTable`, `EuvVirtualList`, `EuvTree`, `EuvAlert`.
- **Forms**: `EuvButton`, `EuvInput`, `EuvTextarea`, `EuvNumberInput`, `EuvField`, `EuvForm`, `EuvLabel`, `EuvSwitch`, `EuvCheckbox`, `EuvRadioGroup`, `EuvSlider`, `EuvSelect`, `EuvRating`, `EuvSegmented`.
- **Navigation**: `EuvSidebarNav`, `EuvTabs`, `EuvBreadcrumb`, `EuvPagination`, `EuvSteps`, `EuvAccordion`, `EuvToc`.
- **Overlays & Hooks**: `EuvModal`, `EuvDrawer`, `EuvSheet`, `EuvPopover`, `EuvTooltip`, `EuvDropdown`, `EuvToast`, `EuvNotification`, `EuvLoading`, plus `use_theme`, `use_i18n`, `use_breakpoint`, `use_router`, `use_gesture`, and `use_window`.
