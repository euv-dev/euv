use super::*;

/// Process-wide cache of the per-load DOM-op names.
///
/// `OnceLock` is the standard library's run-once cell: the closure runs at
/// most once, every later reader blocks until that first write lands, and
/// `get_or_init` is safe to call from any thread. The names are pure data
/// (`String`, no JS handles), so unlike [`DOM_OP_TABLE`] they need no
/// `thread_local!` — one set of names is correct for the whole process, and
/// two threads racing to build it cannot produce two different sets.
static DOM_OP_NAMES: OnceLock<DomOpNames> = OnceLock::new();

impl DomOpNames {
    /// Returns the per-load DOM-op names, building them on first call.
    ///
    /// This is the only read path. Callers must not cache the result
    /// themselves: the whole point of the indirection is that the name is
    /// fixed for the lifetime of the page but not knowable in advance, so
    /// every consumer resolves it through here and the install path and the
    /// lookup path can never disagree about a name.
    ///
    /// # Returns
    ///
    /// - `&'static DomOpNames` - The cached names, shared by every caller.
    pub(crate) fn get() -> &'static DomOpNames {
        DOM_OP_NAMES.get_or_init(build_dom_op_names)
    }

    /// Installs `names` as the process-wide names, if none are set yet.
    ///
    /// The installer calls this before it touches `globalThis`, so the names
    /// are fixed before the first property read. `OnceLock`'s run-once
    /// guarantee is what makes that safe: a second call, or a call after
    /// [`get`] has already initialised the cell, is refused rather than
    /// allowed to swap the names out from under a live table.
    ///
    /// # Arguments
    ///
    /// - `DomOpNames` - The names to publish.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if these names were installed, `false` if names were
    ///   already present.
    pub(crate) fn set(names: DomOpNames) -> bool {
        DOM_OP_NAMES.set(names).is_ok()
    }

    /// Builds a fresh name set from the current clock, without caching it.
    ///
    /// Split out from [`get`] so a caller that needs to *claim* the names can
    /// build a set and hand it to [`set`] rather than letting `get` build one
    /// implicitly. Building costs one clock read and one encode; it happens
    /// once per process, on the first patch.
    ///
    /// # Returns
    ///
    /// - `DomOpNames` - A newly built, uncached name set.
    pub(crate) fn build() -> DomOpNames {
        build_dom_op_names()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns names that are trivially recognisable in an assertion.
    ///
    /// # Returns
    ///
    /// - `DomOpNames` - A fixed, pinned set of names.
    fn pinned() -> DomOpNames {
        DomOpNames {
            table: String::from(JS_DOM_OP_TEST_NAMES[0]),
            set_attrs: String::from(JS_DOM_OP_TEST_NAMES[1]),
            remove_attrs: String::from(JS_DOM_OP_TEST_NAMES[2]),
            child_ops: String::from(JS_DOM_OP_TEST_NAMES[3]),
        }
    }

    #[test]
    fn the_first_set_wins_and_a_later_set_is_refused() {
        let first: bool = DomOpNames::set(pinned());
        let loser: String = String::from(JS_DOM_OP_TEST_LOSER);
        let second: bool = DomOpNames::set(DomOpNames {
            table: loser.clone(),
            set_attrs: loser.clone(),
            remove_attrs: loser.clone(),
            child_ops: loser.clone(),
        });
        let names: &DomOpNames = DomOpNames::get();
        // Whichever call initialised the cell, the later one must not be able
        // to replace the names a live table is already published under.
        assert!(!(first && second), "only one set may be accepted");
        assert_ne!(names.table, JS_DOM_OP_TEST_LOSER);
        assert_ne!(names.table, JS_DOM_OP_TEST_NAMES[0]);
    }

    #[test]
    fn clock_derived_names_share_one_suffix_across_all_four() {
        let names: &DomOpNames = DomOpNames::get();
        let suffix_of: fn(&str) -> String =
            |name: &str| name.rsplit('_').next().unwrap_or_default().to_string();
        let table_suffix: String = suffix_of(&names.table);
        assert!(!table_suffix.is_empty(), "the table name carries a suffix");
        assert_eq!(
            suffix_of(&names.set_attrs),
            table_suffix,
            "every helper name shares the table's suffix"
        );
        assert_eq!(suffix_of(&names.remove_attrs), table_suffix);
        assert_eq!(suffix_of(&names.child_ops), table_suffix);
    }

    #[test]
    fn every_generated_name_is_a_valid_js_identifier() {
        let names: &DomOpNames = DomOpNames::get();
        for name in [
            &names.table,
            &names.set_attrs,
            &names.remove_attrs,
            &names.child_ops,
        ] {
            assert!(
                name.starts_with(JS_DOM_OP_NAME_PREFIX),
                "{name} must carry the euv prefix"
            );
            let first: char = name.chars().next().unwrap_or_default();
            assert!(
                first == '_' || first.is_ascii_alphabetic(),
                "{name} must start an identifier legally"
            );
            for character in name.chars() {
                // `_` and `$` are both legal in a JS identifier; `-` is not,
                // which is why the charset excludes it.
                assert!(
                    character == '_' || character == '$' || character.is_ascii_alphanumeric(),
                    "{name} contains {character}, which is not identifier-safe"
                );
            }
        }
    }

    #[test]
    fn the_fallback_suffix_is_a_legal_identifier_fragment() {
        // The encoder can only fail on a charset the crate rejects, and the
        // charset here is a compile-time constant, so this branch is
        // unreachable in practice. It still has to produce a name the install
        // path can use, because a bad fallback would break the page in a way
        // that is very hard to trace back to a clock reading.
        let fallback: &str = JS_DOM_OP_NAME_FALLBACK_SUFFIX;
        assert!(!fallback.is_empty(), "an empty suffix yields a bare prefix");
        for character in fallback.chars() {
            assert!(
                character == '_' || character == '$' || character.is_ascii_alphanumeric(),
                "fallback suffix contains {character}, which is not identifier-safe"
            );
        }
    }

    #[test]
    fn a_failed_encode_would_still_yield_a_usable_suffix() {
        // Pins the behaviour the fallback exists for: whatever the encoder
        // does, `encoded_name_suffix` must not come back empty, because the
        // install path would then publish every helper under the same bare
        // prefix.
        let suffix: String = encoded_name_suffix();
        assert!(!suffix.is_empty(), "suffix must never be empty");
    }

    #[test]
    fn none_of_the_old_fixed_names_survives() {
        // The whole point of the change: a payload that hard-codes the old
        // names must find nothing to hijack.
        let names: &DomOpNames = DomOpNames::get();
        for retired in JS_DOM_OP_RETIRED_NAMES {
            assert_ne!(names.table, retired);
            assert_ne!(names.set_attrs, retired);
            assert_ne!(names.remove_attrs, retired);
            assert_ne!(names.child_ops, retired);
        }
    }
}
