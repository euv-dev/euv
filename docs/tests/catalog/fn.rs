use super::*;

static CATALOG: &str = r#"{"success":true,"data":{"count":2,"artifacts":[{"name":"docs","fileCount":152,"totalBytes":73616216,"sha256":"03e22c620a4e18868dc8a8365b67e625dbc980a85a91461e117d99e444a0ba40","updatedAt":1795268119},{"name":"euv","fileCount":115,"totalBytes":63758816,"sha256":"7770f71657089f31e417f147943f49c8f7e6563fcadb3dcd959c609918564f2d","updatedAt":1795199985}]}}"#;

#[test]
fn the_catalog_response_is_parsed_into_its_artifacts() {
    let catalog: DocsCatalog = parse_catalog(CATALOG.as_bytes()).unwrap_or_default();
    assert_eq!(catalog.get_artifacts().len(), 2);
    let first: &DocsCatalogArtifact = &catalog.get_artifacts()[0];
    assert_eq!(first.get_name(), "docs");
    assert_eq!(first.get_file_count(), 152);
    assert_eq!(first.get_total_bytes(), 73616216);
    assert_eq!(first.get_updated_at(), 1795268119);
    assert_eq!(
        first.get_sha256(),
        "03e22c620a4e18868dc8a8365b67e625dbc980a85a91461e117d99e444a0ba40"
    );
}

#[test]
fn a_response_without_the_data_envelope_is_still_parsed() {
    let bare: &str = r#"{"count":1,"artifacts":[{"name":"euv","fileCount":1,"totalBytes":2,"sha256":"7770f71657089f31e417f147943f49c8f7e6563fcadb3dcd959c609918564f2d","updatedAt":1}]}"#;
    let catalog: DocsCatalog = parse_catalog(bare.as_bytes()).unwrap_or_default();
    assert_eq!(catalog.get_artifacts().len(), 1);
    assert_eq!(catalog.get_artifacts()[0].get_name(), "euv");
}

#[test]
fn a_response_that_is_not_json_is_refused() {
    assert!(parse_catalog(b"not json").is_err());
}

#[test]
fn a_response_that_is_not_utf8_is_refused() {
    assert!(parse_catalog(&[0xff, 0xfe, 0xfd]).is_err());
}

#[test]
fn a_count_that_does_not_match_the_artifact_count_is_refused() {
    let wrong: &str = r#"{"success":true,"data":{"count":7,"artifacts":[{"name":"euv","fileCount":1,"totalBytes":2,"sha256":"7770f71657089f31e417f147943f49c8f7e6563fcadb3dcd959c609918564f2d","updatedAt":1}]}}"#;
    assert!(parse_catalog(wrong.as_bytes()).is_err());
}

#[test]
fn a_name_that_is_not_a_safe_path_segment_is_refused() {
    for name in [
        "../escape",
        "UPPER",
        "with space",
        "-leading",
        "trailing-",
        "",
    ] {
        let body: String = format!(
            r#"{{"count":1,"artifacts":[{{"name":"{name}","fileCount":1,"totalBytes":2,"sha256":"7770f71657089f31e417f147943f49c8f7e6563fcadb3dcd959c609918564f2d","updatedAt":1}}]}}"#
        );
        assert!(
            parse_catalog(body.as_bytes()).is_err(),
            "the name {name} must be refused"
        );
    }
}

#[test]
fn a_digest_that_is_not_a_sha256_is_refused() {
    let body: &str = r#"{"count":1,"artifacts":[{"name":"euv","fileCount":1,"totalBytes":2,"sha256":"short","updatedAt":1}]}"#;
    assert!(parse_catalog(body.as_bytes()).is_err());
}

#[test]
fn a_digest_with_uppercase_hex_is_refused() {
    let body: &str = r#"{"count":1,"artifacts":[{"name":"euv","fileCount":1,"totalBytes":2,"sha256":"7770F71657089F31E417F147943F49C8F7E6563FCADB3DCD959C609918564F2D","updatedAt":1}]}"#;
    assert!(parse_catalog(body.as_bytes()).is_err());
}

#[test]
fn a_duplicate_name_is_refused() {
    let body: &str = r#"{"count":2,"artifacts":[{"name":"euv","fileCount":1,"totalBytes":2,"sha256":"7770f71657089f31e417f147943f49c8f7e6563fcadb3dcd959c609918564f2d","updatedAt":1},{"name":"euv","fileCount":1,"totalBytes":2,"sha256":"7770f71657089f31e417f147943f49c8f7e6563fcadb3dcd959c609918564f2d","updatedAt":1}]}"#;
    assert!(parse_catalog(body.as_bytes()).is_err());
}

#[test]
fn a_negative_count_is_refused() {
    let body: &str = r#"{"count":1,"artifacts":[{"name":"euv","fileCount":-1,"totalBytes":2,"sha256":"7770f71657089f31e417f147943f49c8f7e6563fcadb3dcd959c609918564f2d","updatedAt":1}]}"#;
    assert!(parse_catalog(body.as_bytes()).is_err());
}

#[test]
fn the_catalog_card_class_is_the_one_the_runtime_validator_asserts_on() {
    assert_eq!(CATALOG_CARD_CLASS, "docs-catalog-card");
}

#[test]
fn every_locale_translates_the_catalog_labels() {
    for tag in supported_locale_tags() {
        for key in [
            DOCS_KEY_CATALOADING,
            DOCS_KEY_CATALOG_ERROR,
            DOCS_KEY_CATALOG_FILES,
            DOCS_KEY_CATALOG_SIZE,
            DOCS_KEY_CATALOG_UPDATED,
        ] {
            let message: Option<String> = lookup_table(tag, key);
            assert!(message.is_some(), "{tag} has no entry for {key}");
            assert!(
                !message.unwrap_or_default().is_empty(),
                "{tag} has an empty entry for {key}"
            );
        }
    }
}

#[test]
fn the_catalog_labels_are_part_of_the_translation_key_list() {
    let keys: Vec<&str> = gate_translation_keys().to_vec();
    for key in [
        DOCS_KEY_CATALOADING,
        DOCS_KEY_CATALOG_ERROR,
        DOCS_KEY_CATALOG_FILES,
        DOCS_KEY_CATALOG_SIZE,
        DOCS_KEY_CATALOG_UPDATED,
    ] {
        assert!(keys.contains(&key), "{key} is not a registered key");
    }
}
