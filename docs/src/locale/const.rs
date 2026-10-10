/// The translation key for the paragraph explaining that the page is
/// locked.
pub(crate) const PW_GATE_KEY_HINT: &str = "docs.password_gate.hint";

/// The translation key for the unlock button's idle label.
pub(crate) const PW_GATE_KEY_IDLE: &str = "docs.password_gate.idle";

/// The translation key for the unlock button's in-flight label.
pub(crate) const PW_GATE_KEY_BUSY: &str = "docs.password_gate.busy";

/// The translation key for the message shown after a wrong password.
pub(crate) const PW_GATE_KEY_ERROR: &str = "docs.password_gate.error";

/// The translation key for the password input's accessible name.
pub(crate) const PW_GATE_KEY_LABEL: &str = "docs.password_gate.label";

/// The translation key for the password input's placeholder.
pub(crate) const PW_GATE_KEY_PLACEHOLDER: &str = "docs.password_gate.placeholder";

/// The Chinese translation table for the password gate. It carries the
/// strings the gate used before it was localised, so the root locale
/// renders exactly what it always did.
pub(crate) const PW_GATE_MESSAGES_ZH: [(&str, &str); 6] = [
    (PW_GATE_KEY_HINT, "本文受密码保护，输入密码后即可查看内容。"),
    (PW_GATE_KEY_IDLE, "解锁"),
    (PW_GATE_KEY_BUSY, "验证中…"),
    (PW_GATE_KEY_ERROR, "密码错误，请重试。"),
    (PW_GATE_KEY_LABEL, "访问密码"),
    (PW_GATE_KEY_PLACEHOLDER, "密码"),
];

/// The English translation table for the password gate, and the fallback
/// every other locale defers to.
pub(crate) const PW_GATE_MESSAGES_EN: [(&str, &str); 6] = [
    (
        PW_GATE_KEY_HINT,
        "This article is password protected. Enter the password to read it.",
    ),
    (PW_GATE_KEY_IDLE, "Unlock"),
    (PW_GATE_KEY_BUSY, "Verifying…"),
    (PW_GATE_KEY_ERROR, "Incorrect password. Please try again."),
    (PW_GATE_KEY_LABEL, "Access password"),
    (PW_GATE_KEY_PLACEHOLDER, "Password"),
];

/// The Japanese translation table for the password gate.
pub(crate) const PW_GATE_MESSAGES_JA: [(&str, &str); 6] = [
    (
        PW_GATE_KEY_HINT,
        "この記事のパスワードで保護されています。パスワードを入力すると内容をご覧できます。",
    ),
    (PW_GATE_KEY_IDLE, "ロック解除"),
    (PW_GATE_KEY_BUSY, "確認中…"),
    (
        PW_GATE_KEY_ERROR,
        "パスワードが違います。もう一度お試しください。",
    ),
    (PW_GATE_KEY_LABEL, "閲覧パスワード"),
    (PW_GATE_KEY_PLACEHOLDER, "パスワード"),
];

/// The Korean translation table for the password gate.
pub(crate) const PW_GATE_MESSAGES_KO: [(&str, &str); 6] = [
    (
        PW_GATE_KEY_HINT,
        "이 문서는 비밀번호로 보호되어 있습니다. 비밀번호를 입력해 주세요.",
    ),
    (PW_GATE_KEY_IDLE, "잠금 해제"),
    (PW_GATE_KEY_BUSY, "확인 중…"),
    (
        PW_GATE_KEY_ERROR,
        "비밀번호가 올바르지 않습니다. 다시 시도해 주세요.",
    ),
    (PW_GATE_KEY_LABEL, "접속 비밀번호"),
    (PW_GATE_KEY_PLACEHOLDER, "비밀번호"),
];

/// The `<html lang>` tag the Chinese locale publishes under.
pub(crate) const PW_GATE_LOCALE_TAG_ZH: &str = "zh-CN";

/// The `<html lang>` tag the English locale publishes under.
pub(crate) const PW_GATE_LOCALE_TAG_EN: &str = "en-US";

/// The `<html lang>` tag the Japanese locale publishes under.
pub(crate) const PW_GATE_LOCALE_TAG_JA: &str = "ja-JP";

/// The `<html lang>` tag the Korean locale publishes under.
pub(crate) const PW_GATE_LOCALE_TAG_KO: &str = "ko-KR";

/// The tag returned when a locale's label is not one of the four known
/// ones. English is the site's source language and its table is the
/// complete one, so it is the only sound default.
pub(crate) const LOCALE_TAG_DEFAULT: &str = PW_GATE_LOCALE_TAG_EN;

/// The display label the content repository gives the Chinese locale.
pub(crate) const LOCALE_LABEL_ZH: &str = "简体中文";

/// The display label the content repository gives the English locale.
pub(crate) const LOCALE_LABEL_EN: &str = "English";

/// The display label the content repository gives the Japanese locale.
pub(crate) const LOCALE_LABEL_JA: &str = "日本語";

/// The display label the content repository gives the Korean locale.
pub(crate) const LOCALE_LABEL_KO: &str = "한국어";

/// The label-to-tag bindings for the four locales the site serves, as
/// `(locale display label, BCP-47 tag)` pairs.
///
/// Kept as one flat table rather than four branches so a locale is added
/// by adding a row, and so the crate keeps exactly one answer to "which
/// language is this route": the router decides *which locale owns the
/// route*, and this table only says what that locale is called. The
/// label is what `build.rs` emits from the content repository's
/// `[[locales]]` frontmatter, so adding a language here cannot make the
/// gate disagree with the content it is translating.
pub(crate) const LOCALE_TAGS: [(&str, &str); 4] = [
    (LOCALE_LABEL_ZH, PW_GATE_LOCALE_TAG_ZH),
    (LOCALE_LABEL_EN, PW_GATE_LOCALE_TAG_EN),
    (LOCALE_LABEL_JA, PW_GATE_LOCALE_TAG_JA),
    (LOCALE_LABEL_KO, PW_GATE_LOCALE_TAG_KO),
];
