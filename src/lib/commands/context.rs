//! `harnessx context` — search for tags and wikilinks across markdown files.
//!
//! Automatically scopes searches to `harnessx/<active-project-id>/`.
//! Uses a built-in recursive search over `.md` and `.json` files.

use std::fs;
use std::io;
use std::path::Path;

use clap::Subcommand;
use serde::Serialize;

use crate::errors::{ParserError, ParserResult};
use crate::models::project::ProjectRegistry;
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum ContextCommand {
    /// Search for tags and wikilinks — returns matching file paths.
    Search {
        /// The search query (e.g. "#tag-name", "[[wikilink]]", or plain text).
        #[arg(long)]
        query: String,
    },
    /// Search with context — returns the paragraph containing each match.
    SearchContext {
        /// The search query (e.g. "#tag-name", "[[wikilink]]", or plain text).
        #[arg(long)]
        query: String,
    },
}

#[derive(Serialize)]
struct ContextMatch {
    file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
}

#[derive(Serialize)]
struct JsonMatch {
    file: String,
    object: serde_json::Value,
}

#[derive(Serialize)]
struct ContextResponse {
    backend: &'static str,
    query: String,
    path: String,
    results: serde_json::Value,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    json_results: Vec<JsonMatch>,
}

impl ContextCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Search { query } => exit_with(run_search(&query, false)),
            Self::SearchContext { query } => exit_with(run_search(&query, true)),
        }
    }
}

// ── Routing ──

/// Resolve the search path from the active project: `harnessx/<id>`.
fn active_project_path() -> ParserResult<String> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    Ok(format!("harnessx/{id}"))
}

fn run_search(query: &str, with_context: bool) -> ParserResult<ContextResponse> {
    let search_path = active_project_path()?;
    run_builtin_search(query, &search_path, with_context)
}

// ── Built-in search backend ──

fn run_builtin_search(
    query: &str,
    search_path: &str,
    with_context: bool,
) -> ParserResult<ContextResponse> {
    let dir = Path::new(search_path);

    if !dir.is_dir() {
        return Err(ParserError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!("directory not found: {search_path}"),
        )));
    }

    let mut matches = Vec::new();
    collect_md_matches(dir, query, with_context, &mut matches)?;

    let mut json_matches = Vec::new();
    collect_json_matches(dir, query, &mut json_matches)?;

    let results = serde_json::to_value(&matches)?;

    Ok(ContextResponse {
        backend: "builtin",
        query: query.to_string(),
        path: search_path.to_string(),
        results,
        json_results: json_matches,
    })
}

/// Recursively walk `dir` for `.md` files that contain `query`.
fn collect_md_matches(
    dir: &Path,
    query: &str,
    with_context: bool,
    results: &mut Vec<ContextMatch>,
) -> ParserResult<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with('.'))
            {
                continue;
            }
            collect_md_matches(&path, query, with_context, results)?;
            continue;
        }

        if !path.extension().is_some_and(|e| e == "md") {
            continue;
        }

        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };

        if !content.contains(query) {
            continue;
        }

        let file_str = path.to_string_lossy().to_string();

        if with_context {
            for paragraph in extract_matching_paragraphs(&content, query) {
                results.push(ContextMatch {
                    file: file_str.clone(),
                    context: Some(paragraph),
                });
            }
        } else {
            results.push(ContextMatch {
                file: file_str,
                context: None,
            });
        }
    }

    Ok(())
}

/// Split content on blank lines and return paragraphs that contain the query.
fn extract_matching_paragraphs(content: &str, query: &str) -> Vec<String> {
    content
        .split("\n\n")
        .filter(|p| p.contains(query))
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

/// Recursively walk `dir` for `.json` files that contain `query`.
///
/// If the file parses as a JSON array, each element whose serialized form
/// contains `query` is returned individually.  Otherwise the entire parsed
/// value is returned.
fn collect_json_matches(
    dir: &Path,
    query: &str,
    results: &mut Vec<JsonMatch>,
) -> ParserResult<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with('.'))
            {
                continue;
            }
            collect_json_matches(&path, query, results)?;
            continue;
        }

        if !path.extension().is_some_and(|e| e == "json") {
            continue;
        }

        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };

        if !content.contains(query) {
            continue;
        }

        let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&content) else {
            continue;
        };

        let file_str = path.to_string_lossy().to_string();

        match parsed {
            serde_json::Value::Array(items) => {
                for item in items {
                    let serialized = serde_json::to_string(&item).unwrap_or_default();
                    if serialized.contains(query) {
                        results.push(JsonMatch {
                            file: file_str.clone(),
                            object: item,
                        });
                    }
                }
            }
            obj => {
                results.push(JsonMatch {
                    file: file_str,
                    object: obj,
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ── extract_matching_paragraphs ──

    #[test]
    fn extracts_matching_paragraph() {
        let content = "first paragraph\n\nsecond has #tag here\n\nthird paragraph";
        let result = extract_matching_paragraphs(content, "#tag");
        assert_eq!(result, vec!["second has #tag here"]);
    }

    #[test]
    fn returns_multiple_matches() {
        let content = "alpha #tag\n\nbeta\n\ngamma #tag";
        let result = extract_matching_paragraphs(content, "#tag");
        assert_eq!(result, vec!["alpha #tag", "gamma #tag"]);
    }

    #[test]
    fn returns_empty_on_no_match() {
        let content = "nothing here\n\nstill nothing";
        let result = extract_matching_paragraphs(content, "#missing");
        assert!(result.is_empty());
    }

    #[test]
    fn trims_whitespace_from_paragraphs() {
        let content = "  spaced #tag  \n\nother";
        let result = extract_matching_paragraphs(content, "#tag");
        assert_eq!(result, vec!["spaced #tag"]);
    }

    #[test]
    fn skips_empty_paragraphs() {
        let content = "\n\n\n\n#tag\n\n\n\n";
        let result = extract_matching_paragraphs(content, "#tag");
        assert_eq!(result, vec!["#tag"]);
    }

    #[test]
    fn multiline_paragraph_returned_whole() {
        let content = "line one\nline two #tag\nline three\n\nunrelated";
        let result = extract_matching_paragraphs(content, "#tag");
        assert_eq!(result, vec!["line one\nline two #tag\nline three"]);
    }

    #[test]
    fn wikilink_in_paragraph() {
        let content = "see [[my_link]] for details\n\nother paragraph";
        let result = extract_matching_paragraphs(content, "[[my_link]]");
        assert_eq!(result, vec!["see [[my_link]] for details"]);
    }

    #[test]
    fn empty_content_returns_empty() {
        assert!(extract_matching_paragraphs("", "#tag").is_empty());
    }

    #[test]
    fn single_paragraph_no_blank_lines() {
        let content = "only one paragraph with #tag inside";
        let result = extract_matching_paragraphs(content, "#tag");
        assert_eq!(result, vec!["only one paragraph with #tag inside"]);
    }

    // ── collect_md_matches (filesystem tests) ──

    /// Create a temp directory with a unique name for each test.
    fn test_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("harnessx_test_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn cleanup(dir: &Path) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn finds_md_files_with_matching_content() {
        let dir = test_dir("finds_md");
        fs::write(dir.join("a.md"), "has #target tag").unwrap();
        fs::write(dir.join("b.md"), "no match here").unwrap();
        fs::write(dir.join("c.md"), "also has #target").unwrap();

        let mut results = Vec::new();
        collect_md_matches(&dir, "#target", false, &mut results).unwrap();

        let files: Vec<&str> = results.iter().map(|m| m.file.as_str()).collect();
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|f| f.ends_with(".md")));
        assert!(results.iter().all(|m| m.context.is_none()));

        cleanup(&dir);
    }

    #[test]
    fn ignores_non_md_files() {
        let dir = test_dir("ignores_non_md");
        fs::write(dir.join("notes.md"), "#target").unwrap();
        fs::write(dir.join("data.txt"), "#target").unwrap();
        fs::write(dir.join("code.rs"), "#target").unwrap();

        let mut results = Vec::new();
        collect_md_matches(&dir, "#target", false, &mut results).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].file.ends_with("notes.md"));

        cleanup(&dir);
    }

    #[test]
    fn recurses_into_subdirectories() {
        let dir = test_dir("recurse_subdir");
        let sub = dir.join("nested");
        fs::create_dir_all(&sub).unwrap();
        fs::write(dir.join("top.md"), "#target").unwrap();
        fs::write(sub.join("deep.md"), "#target").unwrap();

        let mut results = Vec::new();
        collect_md_matches(&dir, "#target", false, &mut results).unwrap();

        assert_eq!(results.len(), 2);

        cleanup(&dir);
    }

    #[test]
    fn skips_hidden_directories() {
        let dir = test_dir("skip_hidden");
        let hidden = dir.join(".hidden");
        fs::create_dir_all(&hidden).unwrap();
        fs::write(dir.join("visible.md"), "#target").unwrap();
        fs::write(hidden.join("secret.md"), "#target").unwrap();

        let mut results = Vec::new();
        collect_md_matches(&dir, "#target", false, &mut results).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].file.ends_with("visible.md"));

        cleanup(&dir);
    }

    #[test]
    fn with_context_returns_paragraphs() {
        let dir = test_dir("context_paragraphs");
        fs::write(
            dir.join("note.md"),
            "intro\n\nparagraph with #target here\n\nconclusion",
        )
        .unwrap();

        let mut results = Vec::new();
        collect_md_matches(&dir, "#target", true, &mut results).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].context.as_deref(),
            Some("paragraph with #target here")
        );

        cleanup(&dir);
    }

    #[test]
    fn empty_directory_returns_no_matches() {
        let dir = test_dir("empty_dir");

        let mut results = Vec::new();
        collect_md_matches(&dir, "#target", false, &mut results).unwrap();

        assert!(results.is_empty());

        cleanup(&dir);
    }

    #[test]
    fn no_matching_content_returns_empty() {
        let dir = test_dir("no_match_content");
        fs::write(dir.join("a.md"), "nothing relevant").unwrap();

        let mut results = Vec::new();
        collect_md_matches(&dir, "#nonexistent", false, &mut results).unwrap();

        assert!(results.is_empty());

        cleanup(&dir);
    }

    // ── run_builtin_search ──

    #[test]
    fn builtin_search_returns_files() {
        let dir = test_dir("fallback_files");
        fs::write(dir.join("note.md"), "has #find-me tag").unwrap();

        let resp = run_builtin_search("#find-me", dir.to_str().unwrap(), false).unwrap();

        assert_eq!(resp.backend, "builtin");
        assert_eq!(resp.query, "#find-me");
        let arr = resp.results.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert!(arr[0].get("file").unwrap().as_str().unwrap().ends_with("note.md"));
        assert!(arr[0].get("context").is_none());

        cleanup(&dir);
    }

    #[test]
    fn builtin_search_context_returns_paragraphs() {
        let dir = test_dir("fallback_context");
        fs::write(
            dir.join("doc.md"),
            "intro\n\nthe [[link]] is here\n\nfooter",
        )
        .unwrap();

        let resp = run_builtin_search("[[link]]", dir.to_str().unwrap(), true).unwrap();

        let arr = resp.results.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(
            arr[0].get("context").unwrap().as_str().unwrap(),
            "the [[link]] is here"
        );

        cleanup(&dir);
    }

    #[test]
    fn builtin_search_nonexistent_dir_errors() {
        let result = run_builtin_search("#tag", "/tmp/harnessx_does_not_exist_xyz", false);
        assert!(result.is_err());
    }

    #[test]
    fn builtin_search_path_included_in_response() {
        let dir = test_dir("fallback_path_field");
        fs::write(dir.join("a.md"), "content").unwrap();

        let resp = run_builtin_search("content", dir.to_str().unwrap(), false).unwrap();
        assert_eq!(resp.path, dir.to_str().unwrap());

        cleanup(&dir);
    }

    // ── collect_json_matches ──

    #[test]
    fn finds_tag_in_json_object() {
        let dir = test_dir("json_object_tag");
        fs::write(
            dir.join("data.json"),
            r##"{"note": "has #target tag", "value": 42}"##,
        )
        .unwrap();

        let mut results = Vec::new();
        collect_json_matches(&dir, "#target", &mut results).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].file.ends_with("data.json"));
        assert_eq!(results[0].object["note"], "has #target tag");
        assert_eq!(results[0].object["value"], 42);

        cleanup(&dir);
    }

    #[test]
    fn returns_individual_matching_elements_from_array() {
        let dir = test_dir("json_array_filter");
        fs::write(
            dir.join("items.json"),
            r##"[
                {"id": 1, "tags": "#target"},
                {"id": 2, "tags": "other"},
                {"id": 3, "tags": "#target again"}
            ]"##,
        )
        .unwrap();

        let mut results = Vec::new();
        collect_json_matches(&dir, "#target", &mut results).unwrap();

        assert_eq!(results.len(), 2);
        let ids: Vec<i64> = results.iter().map(|r| r.object["id"].as_i64().unwrap()).collect();
        assert!(ids.contains(&1));
        assert!(ids.contains(&3));

        cleanup(&dir);
    }

    #[test]
    fn skips_non_matching_json() {
        let dir = test_dir("json_no_match");
        fs::write(dir.join("data.json"), r#"{"note": "nothing here"}"#).unwrap();

        let mut results = Vec::new();
        collect_json_matches(&dir, "#target", &mut results).unwrap();

        assert!(results.is_empty());

        cleanup(&dir);
    }

    #[test]
    fn ignores_non_json_files() {
        let dir = test_dir("json_ignores_other");
        fs::write(dir.join("data.json"), r##"{"tag": "#target"}"##).unwrap();
        fs::write(dir.join("data.txt"), r##"{"tag": "#target"}"##).unwrap();
        fs::write(dir.join("data.md"), r##"{"tag": "#target"}"##).unwrap();

        let mut results = Vec::new();
        collect_json_matches(&dir, "#target", &mut results).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].file.ends_with("data.json"));

        cleanup(&dir);
    }

    #[test]
    fn json_recurses_into_subdirectories() {
        let dir = test_dir("json_recurse");
        let sub = dir.join("nested");
        fs::create_dir_all(&sub).unwrap();
        fs::write(dir.join("top.json"), r##"{"tag": "#target"}"##).unwrap();
        fs::write(sub.join("deep.json"), r##"{"tag": "#target"}"##).unwrap();

        let mut results = Vec::new();
        collect_json_matches(&dir, "#target", &mut results).unwrap();

        assert_eq!(results.len(), 2);

        cleanup(&dir);
    }

    #[test]
    fn json_skips_hidden_directories() {
        let dir = test_dir("json_skip_hidden");
        let hidden = dir.join(".hidden");
        fs::create_dir_all(&hidden).unwrap();
        fs::write(dir.join("visible.json"), r##"{"tag": "#target"}"##).unwrap();
        fs::write(hidden.join("secret.json"), r##"{"tag": "#target"}"##).unwrap();

        let mut results = Vec::new();
        collect_json_matches(&dir, "#target", &mut results).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].file.ends_with("visible.json"));

        cleanup(&dir);
    }

    #[test]
    fn json_skips_invalid_json() {
        let dir = test_dir("json_invalid");
        fs::write(dir.join("bad.json"), "not valid json #target").unwrap();

        let mut results = Vec::new();
        collect_json_matches(&dir, "#target", &mut results).unwrap();

        assert!(results.is_empty());

        cleanup(&dir);
    }

    #[test]
    fn json_wikilink_match() {
        let dir = test_dir("json_wikilink");
        fs::write(
            dir.join("links.json"),
            r#"{"ref": "see [[my_link]] for details"}"#,
        )
        .unwrap();

        let mut results = Vec::new();
        collect_json_matches(&dir, "[[my_link]]", &mut results).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].object["ref"], "see [[my_link]] for details");

        cleanup(&dir);
    }

    #[test]
    fn builtin_search_includes_json_results() {
        let dir = test_dir("fallback_json_integration");
        fs::write(dir.join("note.md"), "has #target tag").unwrap();
        fs::write(dir.join("data.json"), r##"{"tag": "#target"}"##).unwrap();

        let resp = run_builtin_search("#target", dir.to_str().unwrap(), false).unwrap();

        let md_arr = resp.results.as_array().unwrap();
        assert_eq!(md_arr.len(), 1);
        assert!(md_arr[0].get("file").unwrap().as_str().unwrap().ends_with("note.md"));

        assert_eq!(resp.json_results.len(), 1);
        assert!(resp.json_results[0].file.ends_with("data.json"));
        assert_eq!(resp.json_results[0].object["tag"], "#target");

        cleanup(&dir);
    }
}
