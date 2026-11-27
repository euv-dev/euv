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

## System Fonts, Monospace & CJK Shaping

`NativeSystemFont::system()` dispatches glyph rasterization to native OS text engines with distinct proportional, monospace, bold, and italic variants:

- **Apple CoreText + CoreGraphics**: Automatic `CTFontCreateForString` font cascade supporting Latin (`SF Pro`), native monospace (`SF Mono` / `Menlo`), bold/italic traits, and CJK (`PingFang SC`) anti-aliased glyphs.
- **Windows Win32 GDI**: `GetGlyphOutlineW` (`GGO_GRAY8_BITMAP`) smooth coverage extraction with monospace, bold, and italic face selection.
- **Linux / Android FreeType & JNI + Web Canvas**: Native font discovery (`Android` JNI `(IFI)[B` font style flags, FreeType bold/italic/mono faces, Web Canvas2D) with deterministic Unicode/CJK stroke fallback.
- **Packed GPU Glyph Atlas & Zero-Drift Caret**: `GlyphAtlas::pack_rgba_atlas()` shelf-packs cached glyphs into a single RGBA8 texture ready for GPU upload, and `DrawCommand::TextCaret` computes caret X positions from the exact `GlyphAtlas` glyph advances used during text painting.

## Complete `euv-ui` Component Library (60+ Components)

- **Layout & Surface**: `EuvAppShell` (bottom nav, live FPS badge, drag-to-close/expand left drawer), `EuvPageContainer`, `EuvPanel`, `EuvSpace`, `EuvGrid`, `EuvScrollArea`, `EuvSplitter`, `EuvNavbar`, `EuvBottomNav`, `EuvFooter`, `EuvDivider`, `EuvButtonGroup`, `EuvToggle`.
- **Display, Markdown & Data**: `EuvHeader`, `EuvCard`, `EuvStatCard`, `EuvBadge`, `EuvTag`, `EuvAvatar`, `EuvIcon`, `EuvKbd`, `EuvTypography`, `EuvEmpty`, `EuvSkeleton`, `EuvSpinner`, `EuvProgress`, `EuvCodeBlock` (VS Code Eva Light/Dark Bold themes), `EuvMarkdown`, `EuvTimeline`, `EuvList`, `EuvTable`, `EuvVirtualList`, `EuvTree`, `EuvAlert`, `EuvBubble`.
- **Forms & Unified Controls**: `EuvButton`, `EuvInput`, `EuvTextarea`, `EuvNumberInput`, `EuvField`, `EuvForm`, `EuvLabel`, `EuvSwitch`, `EuvCheckbox`, `EuvRadioGroup`, `EuvSlider`, `EuvSelect`, `EuvCombobox`, `EuvNativeSelect`, `EuvDatePicker`, `EuvRating`, `EuvSegmented`.
- **Navigation**: `EuvBottomNav`, `EuvSidebarNav`, `EuvTabs`, `EuvBreadcrumb`, `EuvPagination`, `EuvSteps`, `EuvAccordion`, `EuvToc`.
- **Overlays & Hooks**: `EuvModal`, `EuvDrawer` (left-edge drag grabber), `EuvSheet` (top drag grabber), `EuvPopover`, `EuvTooltip`, `EuvDropdown` (1:1 full-width trigger & menu), `EuvToast`, `EuvNotification`, `EuvLoading`, plus `use_theme`, `use_i18n`, `use_breakpoint`, `use_router`, `use_gesture`, and `use_window`.
