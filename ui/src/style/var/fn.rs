use super::*;

vars! {
    pub c_theme_light {
        // ═══════════════════════════════════════════════════════════════════════
        // Monochrome Design Tokens (Black & White only)
        // ═══════════════════════════════════════════════════════════════════════

        // ─── Surface ───
        background: "#ffffff";
        foreground: "#000000";

        // ─── Secondary / Muted ───
        muted-foreground: "#000000";

        // ─── Accent ───
        accent: "#000000";

        // ─── Border & Input ───
        border: "#000000";
        ring: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Legacy Surface Aliases
        // ═══════════════════════════════════════════════════════════════════════
        bg-overlay: "rgba(0, 0, 0, 0.45)";

        // ─── Text Colors ───
        text-on-accent: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand / Accent (Black for minimalist)
        // ═══════════════════════════════════════════════════════════════════════
        accent: "#000000";
        accent-muted: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Spacing Scale (shadcn/ui Tailwind spacing)
        // ═══════════════════════════════════════════════════════════════════════
        space-2xs: "2px";
        space-xs: "4px";
        space-sm: "8px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        space-2xl: "24px";
        space-3xl: "32px";
        space-4xl: "40px";
        space-7xl: "80px";

        // ═══════════════════════════════════════════════════════════════════════
        // Font Size Scale (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.125rem";
        font-xl: "1.25rem";
        font-2xl: "1.5rem";
        font-3xl: "1.875rem";
        font-4xl: "2.25rem";
        font-5xl: "3rem";
        font-6xl: "3.75rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Transition Durations (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        duration-fast: "0.15s";
        duration-normal: "0.2s";
        duration-slower: "0.4s";
        duration-overlay: "0.2s";
        duration-modal-overlay: "0.15s";
        duration-modal-content: "0.3s";

        // ═══════════════════════════════════════════════════════════════════════
        // Easing Functions (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        ease-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";

        // ═══════════════════════════════════════════════════════════════════════
        // Layout (shadcn/ui aligned)
        // ═══════════════════════════════════════════════════════════════════════
        safe-area-inset-top: "env(safe-area-inset-top, 0px)";
        safe-area-inset-right: "env(safe-area-inset-right, 0px)";
        safe-area-inset-bottom: "env(safe-area-inset-bottom, 0px)";
        safe-area-inset-left: "env(safe-area-inset-left, 0px)";
        padding-shell-top: var!(safe-area-inset-top);
        padding-shell-bottom: var!(safe-area-inset-bottom);
        padding-main-top: "24px";
        padding-main-top-mobile: "16px";
        padding-main-bottom: "24px";
        padding-main-horizontal: "28px";
        padding-main-horizontal-mobile: "16px";
        gap-page-header: "16px";
        gap-page-title: "6px";
        min-height-base: "36px";
        min-height-sm: "36px";
        nav-width: "248px";
        content-max-width: "820px";
        mobile-header-height: "52px";

        // ═══════════════════════════════════════════════════════════════════════
        // Component Spacing Scale
        // ═══════════════════════════════════════════════════════════════════════
        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Page Block Vertical Spacing
        // ═══════════════════════════════════════════════════════════════════════
        page-block-gap: "24px";
        page-block-gap-mobile: "20px";

        // ═══════════════════════════════════════════════════════════════════════
        // Shadows (black alpha only)
        // ═══════════════════════════════════════════════════════════════════════
        shadow-sm: "0 1px 3px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04)";
        shadow-modal: "0 25px 50px -12px rgba(0, 0, 0, 0.18)";
        shadow-drawer: "4px 0 20px rgba(0, 0, 0, 0.08)";
        shadow-accent-sm: "0 1px 3px rgba(0, 0, 0, 0.08)";
        shadow-accent-lg: "0 10px 15px -3px rgba(0, 0, 0, 0.12)";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar
        // ═══════════════════════════════════════════════════════════════════════
        scrollbar-track: "transparent";
        scrollbar-thumb: "rgba(0, 0, 0, 0.15)";
        scrollbar-thumb-hover: "rgba(0, 0, 0, 0.30)";
        scrollbar-thumb-active: "rgba(0, 0, 0, 0.45)";

        // ═══════════════════════════════════════════════════════════════════════
        // Console / VConsole
        // ═══════════════════════════════════════════════════════════════════════
        shadow-console-button: "0 4px 14px rgba(0, 0, 0, 0.15)";
        shadow-console-panel: "0 -8px 32px rgba(0, 0, 0, 0.06)";
    }

    pub c_theme_dark {
        // ═══════════════════════════════════════════════════════════════════════
        // Monochrome Design Tokens (Black & White only)
        // ═══════════════════════════════════════════════════════════════════════

        // ─── Surface ───
        background: "#000000";
        foreground: "#ffffff";

        // ─── Secondary / Muted ───
        muted-foreground: "#ffffff";

        // ─── Accent ───
        accent: "#ffffff";

        // ─── Border & Input ───
        border: "#ffffff";
        ring: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Legacy Surface Aliases
        // ═══════════════════════════════════════════════════════════════════════
        bg-overlay: "rgba(0, 0, 0, 0.60)";

        // ─── Text Colors ───
        text-on-accent: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand / Accent (White for dark minimalist)
        // ═══════════════════════════════════════════════════════════════════════
        accent: "#ffffff";
        accent-muted: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Spacing Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        space-2xs: "2px";
        space-xs: "4px";
        space-sm: "8px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        space-2xl: "24px";
        space-3xl: "32px";
        space-4xl: "40px";
        space-7xl: "80px";

        // ═══════════════════════════════════════════════════════════════════════
        // Font Size Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.125rem";
        font-xl: "1.25rem";
        font-2xl: "1.5rem";
        font-3xl: "1.875rem";
        font-4xl: "2.25rem";
        font-5xl: "3rem";
        font-6xl: "3.75rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Transition Durations (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        duration-fast: "0.15s";
        duration-normal: "0.2s";
        duration-slower: "0.4s";
        duration-overlay: "0.2s";
        duration-modal-overlay: "0.15s";
        duration-modal-content: "0.3s";

        // ═══════════════════════════════════════════════════════════════════════
        // Easing Functions (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        ease-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";

        // ═══════════════════════════════════════════════════════════════════════
        // Layout (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        safe-area-inset-top: "env(safe-area-inset-top, 0px)";
        safe-area-inset-right: "env(safe-area-inset-right, 0px)";
        safe-area-inset-bottom: "env(safe-area-inset-bottom, 0px)";
        safe-area-inset-left: "env(safe-area-inset-left, 0px)";
        padding-shell-top: var!(safe-area-inset-top);
        padding-shell-bottom: var!(safe-area-inset-bottom);
        padding-main-top: "24px";
        padding-main-top-mobile: "16px";
        padding-main-bottom: "24px";
        padding-main-horizontal: "28px";
        padding-main-horizontal-mobile: "16px";
        gap-page-header: "16px";
        gap-page-title: "6px";
        min-height-base: "36px";
        min-height-sm: "36px";
        nav-width: "248px";
        content-max-width: "820px";
        mobile-header-height: "52px";

        // ═══════════════════════════════════════════════════════════════════════
        // Component Spacing Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Page Block Vertical Spacing (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        page-block-gap: "24px";
        page-block-gap-mobile: "20px";

        // ═══════════════════════════════════════════════════════════════════════
        // Shadows (white alpha for dark)
        // ═══════════════════════════════════════════════════════════════════════
        shadow-sm: "0 1px 3px rgba(255, 255, 255, 0.12), 0 1px 2px rgba(255, 255, 255, 0.06)";
        shadow-modal: "0 25px 50px -12px rgba(255, 255, 255, 0.25)";
        shadow-drawer: "4px 0 20px rgba(255, 255, 255, 0.12)";
        shadow-accent-sm: "0 1px 3px rgba(255, 255, 255, 0.15)";
        shadow-accent-lg: "0 10px 15px -3px rgba(255, 255, 255, 0.18)";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar
        // ═══════════════════════════════════════════════════════════════════════
        scrollbar-track: "transparent";
        scrollbar-thumb: "rgba(255, 255, 255, 0.18)";
        scrollbar-thumb-hover: "rgba(255, 255, 255, 0.35)";
        scrollbar-thumb-active: "rgba(255, 255, 255, 0.50)";

        // ═══════════════════════════════════════════════════════════════════════
        // Console / VConsole
        // ═══════════════════════════════════════════════════════════════════════
        shadow-console-button: "0 4px 14px rgba(255, 255, 255, 0.15)";
        shadow-console-panel: "0 -8px 32px rgba(255, 255, 255, 0.08)";
    }
}
