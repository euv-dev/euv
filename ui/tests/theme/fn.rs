use super::*;

#[test]
fn all_seventeen_schemes_resolve_distinct_light_and_dark_tokens() {
    let schemes: [DesignScheme; 17] = DesignScheme::all();
    assert_eq!(schemes.len(), 17);
    for scheme in schemes.into_iter() {
        let light: ThemeTokens = ThemeTokens::resolve(scheme, ThemeMode::Light);
        let dark: ThemeTokens = ThemeTokens::resolve(scheme, ThemeMode::Dark);
        assert_ne!(
            light.get_palette().get_background(),
            dark.get_palette().get_background()
        );
        assert_ne!(
            light.get_palette().get_background(),
            light.get_palette().get_foreground()
        );
        assert!(light.get_geometry().get_control_md() >= 30.0);
        assert!(!scheme.as_str().is_empty());
    }
}

#[test]
fn reactive_ui_theme_state_and_i18n_and_gesture_work() {
    reset_reactive_runtime();
    let state: UiThemeState = UiThemeState::create(DesignScheme::Shadcn, ThemeMode::Light);
    assert_eq!(state.current_tokens().get_mode(), ThemeMode::Light);
    state.toggle_mode();
    assert_eq!(state.current_tokens().get_mode(), ThemeMode::Dark);
    state.select_scheme(DesignScheme::Cupertino);
    assert_eq!(state.current_tokens().get_scheme(), DesignScheme::Cupertino);
    let mut i18n: I18nCatalog = I18nCatalog::create("en".to_string());
    i18n.insert("en".to_string(), "hello".to_string(), "Hello".to_string());
    i18n.insert("zh".to_string(), "hello".to_string(), "你好".to_string());
    assert_eq!(i18n.t("hello"), "Hello");
    let _: bool = i18n.get_locale().set("zh".to_string());
    assert_eq!(i18n.t("hello"), "你好");
    let mut rec: GestureRecognizer = GestureRecognizer::default();
    rec.begin(Point::new(10.0, 10.0));
    rec.update(Point::new(80.0, 10.0));
    assert_eq!(rec.end(Point::new(12.0, 10.0)), Some(GestureKind::Drag));
    rec.begin(Point::new(10.0, 10.0));
    assert_eq!(rec.end(Point::new(12.0, 11.0)), Some(GestureKind::Tap));
}
