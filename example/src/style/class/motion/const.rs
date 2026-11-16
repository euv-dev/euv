/// The example-only class that keeps default app chrome motionless.
pub(crate) const EXAMPLE_MOTION_POLICY_CLASS: &str = "c_example_motion_policy";

/// Explicit themes keep their own policy; default chrome remains motionless.
pub(crate) const EXAMPLE_MOTION_POLICY_STYLES: [&str; 2] = [
    "",
    "animation:none!important;transition:none!important;scroll-behavior:auto!important;",
];

/// Selects the default chrome policy at compilation, without theme dispatch in views.
#[cfg(not(any(
    feature = "theme-antdesign",
    feature = "theme-bootstrap",
    feature = "theme-cupertino",
    feature = "theme-custom",
    feature = "theme-daisyui",
    feature = "theme-fluent",
    feature = "theme-geist",
    feature = "theme-heroui",
    feature = "theme-linear",
    feature = "theme-mantine",
    feature = "theme-material",
    feature = "theme-monochrome",
    feature = "theme-notion",
    feature = "theme-park",
    feature = "theme-radix",
    feature = "theme-shadcn",
    feature = "theme-stripe",
    feature = "theme-supabase"
)))]
pub(crate) const EXAMPLE_MOTION_POLICY_STYLE: &str = EXAMPLE_MOTION_POLICY_STYLES[1];

/// Explicit theme builds retain their own component transitions.
#[cfg(any(
    feature = "theme-antdesign",
    feature = "theme-bootstrap",
    feature = "theme-cupertino",
    feature = "theme-custom",
    feature = "theme-daisyui",
    feature = "theme-fluent",
    feature = "theme-geist",
    feature = "theme-heroui",
    feature = "theme-linear",
    feature = "theme-mantine",
    feature = "theme-material",
    feature = "theme-monochrome",
    feature = "theme-notion",
    feature = "theme-park",
    feature = "theme-radix",
    feature = "theme-shadcn",
    feature = "theme-stripe",
    feature = "theme-supabase"
))]
pub(crate) const EXAMPLE_MOTION_POLICY_STYLE: &str = EXAMPLE_MOTION_POLICY_STYLES[0];

/// Descendant selectors preserve the same class scope for every affected element.
pub(crate) const EXAMPLE_MOTION_POLICY_SELECTORS: [&str; 3] = [" *", " *::before", " *::after"];
