//! Unit tests for the language registry.

use super::*;
use std::path::Path;

#[test]
fn detects_rust_by_extension() {
    let language =
        detect_language(Path::new("src/main.rs")).unwrap();
    assert_eq!(language.id, "rust");
    assert_eq!(language.engine, EngineKind::Rust);
}

#[test]
fn detects_toml_by_extension() {
    let language = detect_language(Path::new("Cargo.toml")).unwrap();
    assert_eq!(language.id, "toml");
}

#[test]
fn detects_cargo_lock_by_filename() {
    let language = detect_language(Path::new("Cargo.lock")).unwrap();
    assert_eq!(language.id, "toml");
}

#[test]
fn detects_html_by_extension() {
    let language = detect_language(Path::new("index.html")).unwrap();
    assert_eq!(language.id, "html");
    assert_eq!(language.engine, EngineKind::Markup);
}

#[test]
fn detects_javascript_by_extension() {
    let language = detect_language(Path::new("app.js")).unwrap();
    assert_eq!(language.id, "javascript");
    assert_eq!(language.engine, EngineKind::CStyle);
}

#[test]
fn detects_typescript_by_extension() {
    let language = detect_language(Path::new("app.ts")).unwrap();
    assert_eq!(language.id, "typescript");
}

#[test]
fn detects_kotlin_script_by_extension() {
    let language =
        detect_language(Path::new("build.gradle.kts")).unwrap();
    assert_eq!(language.id, "kotlin");
}

#[test]
fn detects_gradle_groovy_by_extension() {
    let language =
        detect_language(Path::new("build.gradle")).unwrap();
    assert_eq!(language.id, "groovy");
}

#[test]
fn detects_dockerfile_by_filename() {
    let language = detect_language(Path::new("Dockerfile")).unwrap();
    assert_eq!(language.id, "dockerfile");
}

#[test]
fn detects_dockerfile_variant_by_prefix() {
    let language =
        detect_language(Path::new("Dockerfile.production")).unwrap();
    assert_eq!(language.id, "dockerfile");
}

#[test]
fn detects_makefile_by_filename() {
    let language = detect_language(Path::new("Makefile")).unwrap();
    assert_eq!(language.id, "makefile");
}

#[test]
fn detects_dotenv_by_filename() {
    let language = detect_language(Path::new(".env")).unwrap();
    assert_eq!(language.id, "properties");
}

#[test]
fn detects_dotenv_variant_by_prefix() {
    let language =
        detect_language(Path::new(".env.production")).unwrap();
    assert_eq!(language.id, "properties");
}

#[test]
fn detects_gitignore_by_filename() {
    let language = detect_language(Path::new(".gitignore")).unwrap();
    assert_eq!(language.id, "gitignore");
}

#[test]
fn detects_svg_as_svg() {
    let language = detect_language(Path::new("icon.svg")).unwrap();
    // svg is registered before the generic xml entry.
    assert_eq!(language.id, "svg");
}

#[test]
fn detects_xml_by_extension() {
    let language =
        detect_language(Path::new("AndroidManifest.xml")).unwrap();
    assert_eq!(language.id, "xml");
}

#[test]
fn detects_android_layout_as_xml() {
    let language =
        detect_language(Path::new("res/layout/activity_main.xml"))
            .unwrap();
    assert_eq!(language.id, "xml");
}

#[test]
fn detects_aidl_by_extension() {
    let language =
        detect_language(Path::new("MyService.aidl")).unwrap();
    assert_eq!(language.id, "aidl");
    assert_eq!(language.category, Category::Android);
}

#[test]
fn detects_proguard_by_extension() {
    let language =
        detect_language(Path::new("proguard-rules.pro")).unwrap();
    assert_eq!(language.id, "proguard");
}

#[test]
fn detects_smali_by_extension() {
    let language =
        detect_language(Path::new("MainActivity.smali")).unwrap();
    assert_eq!(language.id, "smali");
}

#[test]
fn detects_yaml_by_extension() {
    let language = detect_language(Path::new("ci.yml")).unwrap();
    assert_eq!(language.id, "yaml");
    assert!(!language.single_line);
}

#[test]
fn detects_json_by_extension() {
    let language =
        detect_language(Path::new("package.json")).unwrap();
    assert_eq!(language.id, "json");
    assert!(language.single_line);
}

#[test]
fn detects_sql_by_extension() {
    let language = detect_language(Path::new("schema.sql")).unwrap();
    assert_eq!(language.id, "sql");
}

#[test]
fn detects_graphql_by_extension() {
    let language =
        detect_language(Path::new("schema.graphql")).unwrap();
    assert_eq!(language.id, "graphql");
}

#[test]
fn finds_language_by_alias() {
    assert_eq!(find_language("js").unwrap().id, "javascript");
    assert_eq!(find_language("bash").unwrap().id, "shell");
    assert_eq!(find_language("objc").unwrap().id, "objectivec");
}

#[test]
fn finds_language_case_insensitively() {
    assert_eq!(
        find_language("JavaScript").unwrap().id,
        "javascript"
    );
    assert_eq!(find_language("KOTLIN").unwrap().id, "kotlin");
}

#[test]
fn returns_none_for_unknown_extension() {
    assert!(detect_language(Path::new("archive.unknown")).is_none());
}

#[test]
fn detects_python_as_unsupported() {
    assert!(detect_language(Path::new("script.py")).is_none());
}

#[test]
fn detects_markdown_as_unsupported() {
    assert!(detect_language(Path::new("README.md")).is_none());
}

#[test]
fn excluded_types_are_listed() {
    assert!(
        EXCLUDED_TYPES
            .iter()
            .any(|entry| entry.name == "Python")
    );
    assert!(
        EXCLUDED_TYPES
            .iter()
            .any(|entry| entry.name.contains("Markdown"))
    );
}

#[test]
fn comment_style_renders_line_comment() {
    let rendered = CommentStyle::Line("//")
        .render("src/main.rs")
        .unwrap();
    assert_eq!(rendered, "// src/main.rs");
}

#[test]
fn comment_style_renders_block_comment() {
    let rendered = CommentStyle::Block("<!--", "-->")
        .render("index.html")
        .unwrap();
    assert_eq!(rendered, "<!-- index.html -->");
}

#[test]
fn comment_style_none_renders_nothing() {
    assert!(
        CommentStyle::None
            .render("data.bin")
            .is_none()
    );
}

#[test]
fn source_sniffing_detects_xml_config() {
    let path = Path::new("app.config");
    let source = "<?xml version=\"1.0\"?><configuration/>";
    let language =
        detect_language_from_source(path, source).unwrap();
    assert_eq!(language.id, "xml");
}

#[test]
fn source_sniffing_detects_ini_config() {
    let path = Path::new("app.config");
    let source = "[section]\nkey=value\n";
    let language =
        detect_language_from_source(path, source).unwrap();
    assert_eq!(language.id, "ini");
}

#[test]
fn source_sniffing_detects_objective_c() {
    let path = Path::new("view.m");
    let source =
        "#import <UIKit/UIKit.h>\n@implementation View\n@end";
    let language =
        detect_language_from_source(path, source).unwrap();
    assert_eq!(language.id, "objectivec");
}

#[test]
fn source_sniffing_rejects_matlab_m_files() {
    let path = Path::new("plot.m");
    let source = "function y = plot(x)\ny = x * 2;\nend";
    assert!(detect_language_from_source(path, source).is_none());
}

#[test]
fn every_language_has_an_extension_or_filename() {
    for language in LANGUAGES {
        assert!(
            !language.extensions.is_empty()
                || !language.filenames.is_empty()
                || !language.filename_prefixes.is_empty(),
            "{} has no way to be detected",
            language.id
        );
    }
}

#[test]
fn every_language_has_a_unique_id() {
    let mut seen = Vec::new();
    for language in LANGUAGES {
        assert!(
            !seen.contains(&language.id),
            "duplicate language id: {}",
            language.id
        );
        seen.push(language.id);
    }
}

#[test]
fn supported_extension_summary_mentions_common_types() {
    let summary = supported_extension_summary();
    for expected in [".rs", ".toml", ".html", ".js", ".kt", ".java"]
    {
        assert!(
            summary.contains(expected),
            "summary is missing {}",
            expected
        );
    }
}

#[test]
fn type_listing_includes_supported_and_excluded() {
    let listing = type_listing();
    assert!(listing.contains("Supported types"));
    assert!(listing.contains("Excluded types"));
    assert!(listing.contains("html"));
    assert!(listing.contains("Python"));
    assert!(listing.contains("indentation"));
}
