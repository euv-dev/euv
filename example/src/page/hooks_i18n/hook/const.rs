/// Default locale tag for the i18n demo.
pub(crate) const HOOKS_I18N_DEFAULT_LOCALE: &str = "en";

/// Secondary locale tag the demo can switch into.
pub(crate) const HOOKS_I18N_OTHER_LOCALE: &str = "zh-CN";

/// English-language button label.
pub(crate) const HOOKS_I18N_LABEL_EN: &str = "English";

/// "Other" (Chinese) language button label.
pub(crate) const HOOKS_I18N_LABEL_OTHER: &str = "中文";

/// Translation key for the greeting message.
pub(crate) const HOOKS_I18N_KEY_GREETING: &str = "greeting";

/// Translation key for the farewell message.
pub(crate) const HOOKS_I18N_KEY_FAREWELL: &str = "farewell";

/// English message for [`HOOKS_I18N_KEY_GREETING`].
pub(crate) const HOOKS_I18N_MESSAGE_EN_GREETING: &str = "Hello, world!";

/// English message for [`HOOKS_I18N_KEY_FAREWELL`].
pub(crate) const HOOKS_I18N_MESSAGE_EN_FAREWELL: &str = "Goodbye, world!";

/// Chinese message for [`HOOKS_I18N_KEY_GREETING`].
pub(crate) const HOOKS_I18N_MESSAGE_ZH_GREETING: &str = "你好,世界!";

/// Chinese message for [`HOOKS_I18N_KEY_FAREWELL`].
pub(crate) const HOOKS_I18N_MESSAGE_ZH_FAREWELL: &str = "再见,世界!";

/// English (`en`) translation table — `&'static` tuples are the
/// only form `i18n_register` can accept without leaking. The
/// underlying `I18n::add_messages` does the
/// `&str -> String` round-trip on the caller's behalf.
pub(crate) const HOOKS_I18N_EN_MESSAGES: [(&str, &str); 2] = [
    (HOOKS_I18N_KEY_GREETING, HOOKS_I18N_MESSAGE_EN_GREETING),
    (HOOKS_I18N_KEY_FAREWELL, HOOKS_I18N_MESSAGE_EN_FAREWELL),
];

/// `zh-CN` translation table.
pub(crate) const HOOKS_I18N_ZH_MESSAGES: [(&str, &str); 2] = [
    (HOOKS_I18N_KEY_GREETING, HOOKS_I18N_MESSAGE_ZH_GREETING),
    (HOOKS_I18N_KEY_FAREWELL, HOOKS_I18N_MESSAGE_ZH_FAREWELL),
];
