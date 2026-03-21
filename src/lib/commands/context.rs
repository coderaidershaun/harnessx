//! `harnessx context` — search for tags and wikilinks across markdown files.
//!
//! Automatically scopes searches to `harnessx/<active-project-id>/`.
//! Checks for the Obsidian CLI on each invocation. If available, delegates to
//! `obsidian search` / `obsidian search:context`. Otherwise, falls back to a
//! built-in recursive search over `.md` files.

use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

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
struct ContextResponse {
    backend: &'static str,
    query: String,
    path: String,
    results: serde_json::Value,
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

/// Check whether the Obsidian **CLI** (`obsidian-cli`) is on PATH.
///
/// On macOS the Obsidian *desktop app* installs a binary at
/// `/Applications/Obsidian.app/…/obsidian` which `which` will find, but
/// that binary is not the CLI tool and will hang when called with search
/// arguments.  We filter it out by rejecting paths inside `.app` bundles.
fn obsidian_available() -> bool {
    let output = Command::new("which")
        .arg("obsidian")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let path = String::from_utf8_lossy(&out.stdout);
            !path.trim().contains(".app/")
        }
        _ => false,
    }
}

fn run_search(query: &str, with_context: bool) -> ParserResult<ContextResponse> {
    let search_path = active_project_path()?;

    if obsidian_available() {
        run_obsidian_search(query, &search_path, with_context)
    } else {
        run_fallback_search(query, &search_path, with_context)
    }
}

// ── Obsidian backend ──

/// Translate a user-facing query into the Obsidian CLI query syntax.
///
/// - `#tag`       → `tag:#tag` (search) or `section:(#tag)` (search:context)
/// - `[[link]]`   → `/\[\[link\]\]/` (regex match)
/// - plain text   → passed through unchanged
fn build_obsidian_query(query: &str, with_context: bool) -> String {
    if let Some(inner) = query
        .strip_prefix("[[")
        .and_then(|s| s.strip_suffix("]]"))
    {
        format!("/\\[\\[{inner}\\]\\]/")
    } else if query.starts_with('#') {
        if with_context {
            format!("section:({query})")
        } else {
            format!("tag:{query}")
        }
    } else {
        query.to_owned()
    }
}

fn run_obsidian_search(
    query: &str,
    search_path: &str,
    with_context: bool,
) -> ParserResult<ContextResponse> {
    let obsidian_query = build_obsidian_query(query, with_context);
    let subcommand = if with_context {
        "search:context"
    } else {
        "search"
    };

    let mut cmd = Command::new("obsidian");
    cmd.arg(subcommand);
    cmd.arg(format!("path={search_path}"));
    cmd.arg(format!("query={obsidian_query}"));
    cmd.arg("format=json");

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ParserError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("obsidian CLI failed: {}", stderr.trim()),
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let results: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|_| serde_json::Value::String(stdout.trim().to_string()));

    Ok(ContextResponse {
        backend: "obsidian",
        query: query.to_string(),
        path: search_path.to_string(),
        results,
    })
}

// ── Fallback backend (no Obsidian CLI) ──

fn run_fallback_search(
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

    let results = serde_json::to_value(&matches)?;

    Ok(ContextResponse {
        backend: "fallback",
        query: query.to_string(),
        path: search_path.to_string(),
        results,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ── build_obsidian_query ──

    #[test]
    fn tag_search_uses_tag_prefix() {
        assert_eq!(build_obsidian_query("#my-tag", false), "tag:#my-tag");
    }

    #[test]
    fn tag_context_uses_section_wrapper() {
        assert_eq!(
            build_obsidian_query("#my-tag", true),
            "section:(#my-tag)"
        );
    }

    #[test]
    fn wikilink_becomes_regex() {
        assert_eq!(
            build_obsidian_query("[[some_link]]", false),
            "/\\[\\[some_link\\]\\]/"
        );
    }

    #[test]
    fn wikilink_context_also_becomes_regex() {
        assert_eq!(
            build_obsidian_query("[[link]]", true),
            "/\\[\\[link\\]\\]/"
        );
    }

    #[test]
    fn plain_text_passes_through() {
        assert_eq!(build_obsidian_query("hello world", false), "hello world");
        assert_eq!(build_obsidian_query("hello world", true), "hello world");
    }

    #[test]
    fn partial_brackets_are_plain_text() {
        assert_eq!(build_obsidian_query("[[no-close", false), "[[no-close");
        assert_eq!(build_obsidian_query("no-open]]", false), "no-open]]");
    }

    #[test]
    fn tag_with_colons_preserves_namespacing() {
        assert_eq!(
            build_obsidian_query("#project::scope", false),
            "tag:#project::scope"
        );
        assert_eq!(
            build_obsidian_query("#project::scope", true),
            "section:(#project::scope)"
        );
    }

    #[test]
    fn wikilink_with_colons_preserves_namespacing() {
        assert_eq!(
            build_obsidian_query("[[project::some_link]]", false),
            "/\\[\\[project::some_link\\]\\]/"
        );
    }

    #[test]
    fn empty_wikilink_becomes_regex() {
        assert_eq!(build_obsidian_query("[[]]", false), "/\\[\\[\\]\\]/");
    }

    #[test]
    fn hash_only_treated_as_tag() {
        assert_eq!(build_obsidian_query("#", false), "tag:#");
    }

    #[test]
    fn empty_string_passes_through() {
        assert_eq!(build_obsidian_query("", false), "");
    }

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

    // ── run_fallback_search ──

    #[test]
    fn fallback_search_returns_files() {
        let dir = test_dir("fallback_files");
        fs::write(dir.join("note.md"), "has #find-me tag").unwrap();

        let resp = run_fallback_search("#find-me", dir.to_str().unwrap(), false).unwrap();

        assert_eq!(resp.backend, "fallback");
        assert_eq!(resp.query, "#find-me");
        let arr = resp.results.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert!(arr[0].get("file").unwrap().as_str().unwrap().ends_with("note.md"));
        assert!(arr[0].get("context").is_none());

        cleanup(&dir);
    }

    #[test]
    fn fallback_search_context_returns_paragraphs() {
        let dir = test_dir("fallback_context");
        fs::write(
            dir.join("doc.md"),
            "intro\n\nthe [[link]] is here\n\nfooter",
        )
        .unwrap();

        let resp = run_fallback_search("[[link]]", dir.to_str().unwrap(), true).unwrap();

        let arr = resp.results.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(
            arr[0].get("context").unwrap().as_str().unwrap(),
            "the [[link]] is here"
        );

        cleanup(&dir);
    }

    #[test]
    fn fallback_search_nonexistent_dir_errors() {
        let result = run_fallback_search("#tag", "/tmp/harnessx_does_not_exist_xyz", false);
        assert!(result.is_err());
    }

    #[test]
    fn fallback_search_path_included_in_response() {
        let dir = test_dir("fallback_path_field");
        fs::write(dir.join("a.md"), "content").unwrap();

        let resp = run_fallback_search("content", dir.to_str().unwrap(), false).unwrap();
        assert_eq!(resp.path, dir.to_str().unwrap());

        cleanup(&dir);
    }
}
