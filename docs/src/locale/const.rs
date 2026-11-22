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

/// The localized accessible name for opening the navigation drawer.
pub(crate) const DOCS_KEY_DRAWER_OPEN: &str = "docs.drawer.open";
/// The localized accessible name for closing the navigation drawer.
pub(crate) const DOCS_KEY_DRAWER_CLOSE: &str = "docs.drawer.close";
/// The localized accessible name of the modal navigation dialog.
pub(crate) const DOCS_KEY_DRAWER_TITLE: &str = "docs.drawer.title";
/// The localized accessible name and tooltip of the theme toggle.
pub(crate) const DOCS_KEY_THEME_TOGGLE: &str = "docs.theme.toggle";
/// The localized tooltip of the language control.
pub(crate) const DOCS_KEY_LOCALE_TITLE: &str = "docs.locale.title";
/// The localized accessible name of documentation search.
pub(crate) const DOCS_KEY_SEARCH_LABEL: &str = "docs.search.label";
/// The localized instruction announced for an empty query.
pub(crate) const DOCS_KEY_SEARCH_EMPTY: &str = "docs.search.empty";
/// The localized announcement for a query without matches.
pub(crate) const DOCS_KEY_SEARCH_NONE: &str = "docs.search.none";
/// The localized search field placeholder.
pub(crate) const DOCS_KEY_SEARCH_PLACEHOLDER: &str = "docs.search.placeholder";
/// The localized result count template.
pub(crate) const DOCS_KEY_SEARCH_COUNT: &str = "docs.search.count";
/// The substitution token in localized result count templates.
pub(crate) const DOCS_SEARCH_COUNT_TOKEN: &str = "{count}";
/// The localized message shown while the catalog is loading.
pub(crate) const DOCS_KEY_CATALOADING: &str = "docs.catalog.loading";
/// The localized message shown when the catalog could not be loaded.
pub(crate) const DOCS_KEY_CATALOG_ERROR: &str = "docs.catalog.error";
/// The localized label of the file count field.
pub(crate) const DOCS_KEY_CATALOG_FILES: &str = "docs.catalog.files";
/// The localized label of the total size field.
pub(crate) const DOCS_KEY_CATALOG_SIZE: &str = "docs.catalog.size";
/// The localized label of the update time field.
pub(crate) const DOCS_KEY_CATALOG_UPDATED: &str = "docs.catalog.updated";
/// The localized label of the button that retries a failed catalog load.
pub(crate) const DOCS_KEY_CATALOG_RETRY: &str = "docs.catalog.retry";

/// The Chinese translation table for the password gate. It carries the
/// strings the gate used before it was localised, so the root locale
/// renders exactly what it always did.
pub(crate) const PW_GATE_MESSAGES_ZH: [(&str, &str); 22] = [
    (PW_GATE_KEY_HINT, "本文受密码保护，输入密码后即可查看内容。"),
    (PW_GATE_KEY_IDLE, "解锁"),
    (PW_GATE_KEY_BUSY, "验证中…"),
    (PW_GATE_KEY_ERROR, "密码错误，请重试。"),
    (PW_GATE_KEY_LABEL, "访问密码"),
    (PW_GATE_KEY_PLACEHOLDER, "密码"),
    (DOCS_KEY_DRAWER_OPEN, "打开导航菜单"),
    (DOCS_KEY_DRAWER_CLOSE, "关闭导航菜单"),
    (DOCS_KEY_DRAWER_TITLE, "文档导航"),
    (DOCS_KEY_THEME_TOGGLE, "切换主题"),
    (DOCS_KEY_LOCALE_TITLE, "语言"),
    (DOCS_KEY_SEARCH_LABEL, "搜索文档"),
    (DOCS_KEY_SEARCH_EMPTY, "输入关键词搜索文档"),
    (DOCS_KEY_SEARCH_NONE, "没有匹配的页面"),
    (DOCS_KEY_SEARCH_PLACEHOLDER, "搜索文档…"),
    (DOCS_KEY_SEARCH_COUNT, "找到 {count} 个匹配页面"),
    (DOCS_KEY_CATALOADING, "正在加载项目…"),
    (DOCS_KEY_CATALOG_ERROR, "项目列表加载失败，请稍后重试。"),
    (DOCS_KEY_CATALOG_FILES, "文件数"),
    (DOCS_KEY_CATALOG_SIZE, "总大小"),
    (DOCS_KEY_CATALOG_UPDATED, "更新时间"),
    (DOCS_KEY_CATALOG_RETRY, "重试"),
];

/// The English translation table for the password gate, and the fallback
/// every other locale defers to.
pub(crate) const PW_GATE_MESSAGES_EN: [(&str, &str); 22] = [
    (
        PW_GATE_KEY_HINT,
        "This article is password protected. Enter the password to read it.",
    ),
    (PW_GATE_KEY_IDLE, "Unlock"),
    (PW_GATE_KEY_BUSY, "Verifying…"),
    (PW_GATE_KEY_ERROR, "Incorrect password. Please try again."),
    (PW_GATE_KEY_LABEL, "Access password"),
    (PW_GATE_KEY_PLACEHOLDER, "Password"),
    (DOCS_KEY_DRAWER_OPEN, "Open navigation menu"),
    (DOCS_KEY_DRAWER_CLOSE, "Close navigation menu"),
    (DOCS_KEY_DRAWER_TITLE, "Documentation navigation"),
    (DOCS_KEY_THEME_TOGGLE, "Toggle theme"),
    (DOCS_KEY_LOCALE_TITLE, "Language"),
    (DOCS_KEY_SEARCH_LABEL, "Search documentation"),
    (DOCS_KEY_SEARCH_EMPTY, "Type to search documentation"),
    (DOCS_KEY_SEARCH_NONE, "No matching pages"),
    (DOCS_KEY_SEARCH_PLACEHOLDER, "Search documentation..."),
    (DOCS_KEY_SEARCH_COUNT, "{count} matching pages"),
    (DOCS_KEY_CATALOADING, "Loading projects…"),
    (
        DOCS_KEY_CATALOG_ERROR,
        "The project list could not be loaded. Please try again later.",
    ),
    (DOCS_KEY_CATALOG_FILES, "Files"),
    (DOCS_KEY_CATALOG_SIZE, "Total size"),
    (DOCS_KEY_CATALOG_UPDATED, "Updated"),
    (DOCS_KEY_CATALOG_RETRY, "Retry"),
];

/// The Japanese translation table for the password gate.
pub(crate) const PW_GATE_MESSAGES_JA: [(&str, &str); 22] = [
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
    (DOCS_KEY_DRAWER_OPEN, "ナビゲーションメニューを開く"),
    (DOCS_KEY_DRAWER_CLOSE, "ナビゲーションメニューを閉じる"),
    (DOCS_KEY_DRAWER_TITLE, "ドキュメントナビゲーション"),
    (DOCS_KEY_THEME_TOGGLE, "テーマを切り替える"),
    (DOCS_KEY_LOCALE_TITLE, "言語"),
    (DOCS_KEY_SEARCH_LABEL, "ドキュメントを検索"),
    (
        DOCS_KEY_SEARCH_EMPTY,
        "キーワードを入力してドキュメントを検索",
    ),
    (DOCS_KEY_SEARCH_NONE, "一致するページがありません"),
    (DOCS_KEY_SEARCH_PLACEHOLDER, "ドキュメントを検索…"),
    (DOCS_KEY_SEARCH_COUNT, "{count} 件のページが見つかりました"),
    (DOCS_KEY_CATALOADING, "プロジェクトを読み込み中…"),
    (
        DOCS_KEY_CATALOG_ERROR,
        "プロジェクト一覧を読み込めませんでした。後でもう一度お試しください。",
    ),
    (DOCS_KEY_CATALOG_FILES, "ファイル数"),
    (DOCS_KEY_CATALOG_SIZE, "合計サイズ"),
    (DOCS_KEY_CATALOG_UPDATED, "更新日時"),
    (DOCS_KEY_CATALOG_RETRY, "再試行"),
];

/// The Korean translation table for the password gate.
pub(crate) const PW_GATE_MESSAGES_KO: [(&str, &str); 22] = [
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
    (DOCS_KEY_DRAWER_OPEN, "탐색 메뉴 열기"),
    (DOCS_KEY_DRAWER_CLOSE, "탐색 메뉴 닫기"),
    (DOCS_KEY_DRAWER_TITLE, "문서 탐색"),
    (DOCS_KEY_THEME_TOGGLE, "테마 전환"),
    (DOCS_KEY_LOCALE_TITLE, "언어"),
    (DOCS_KEY_SEARCH_LABEL, "문서 검색"),
    (DOCS_KEY_SEARCH_EMPTY, "검색어를 입력하여 문서 검색"),
    (DOCS_KEY_SEARCH_NONE, "일치하는 페이지가 없습니다"),
    (DOCS_KEY_SEARCH_PLACEHOLDER, "문서 검색…"),
    (DOCS_KEY_SEARCH_COUNT, "일치하는 페이지 {count}개"),
    (DOCS_KEY_CATALOADING, "프로젝트를 불러오는 중…"),
    (
        DOCS_KEY_CATALOG_ERROR,
        "프로젝트 목록을 불러오지 못했습니다. 나중에 다시 시도해 주세요.",
    ),
    (DOCS_KEY_CATALOG_FILES, "파일 수"),
    (DOCS_KEY_CATALOG_SIZE, "전체 크기"),
    (DOCS_KEY_CATALOG_UPDATED, "업데이트 시각"),
    (DOCS_KEY_CATALOG_RETRY, "다시 시도"),
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
