/// All 17 built-in cross-platform design languages expressing distinct color, radius, and control geometry.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DesignScheme {
    /// Crisp hairline-bordered minimal UI language.
    #[default]
    Shadcn,
    /// Apple HIG inspired soft-corner blue accent language.
    Cupertino,
    /// Google Material 3 rounded elevation language.
    Material,
    /// Microsoft Fluent acrylic/subtle-border desktop language.
    Fluent,
    /// Enterprise Ant Design high-clarity language.
    AntDesign,
    /// Classic Bootstrap balanced component language.
    Bootstrap,
    /// Playful high-contrast DaisyUI pill/rounded language.
    DaisyUi,
    /// Vercel Geist crisp monochrome + blue language.
    Geist,
    /// Modern vibrant HeroUI language.
    HeroUi,
    /// Precision dark-leaning Linear indigo workspace language.
    Linear,
    /// Clean Mantine teal/blue component language.
    Mantine,
    /// Strict high-contrast monochrome ink language.
    Monochrome,
    /// Editorial warm-neutral Notion document language.
    Notion,
    /// Structured Park UI slate language.
    Park,
    /// Accessible Radix UI crisp indigo language.
    Radix,
    /// Precision Stripe blurple fintech language.
    Stripe,
    /// Developer-focused Supabase emerald language.
    Supabase,
}
