//! Find live Claude Code sessions by custom title.

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

use clap::Subcommand;
use serde::Serialize;

use crate::errors::ParserResult;
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum SessionCommand {
    /// Find live Claude Code sessions matching a custom title.
    Find {
        /// The custom session title to search for.
        name: String,
    },
}

#[derive(Serialize, Debug, PartialEq)]
pub struct LiveSession {
    pub pid: String,
    pub session_id: String,
    pub project: String,
}

impl SessionCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Find { name } => exit_with(find_live_sessions(&name)),
        }
    }
}

pub fn find_live_sessions(name: &str) -> ParserResult<Vec<LiveSession>> {
    let home = std::env::var("HOME").map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "HOME environment variable not set",
        )
    })?;

    let projects_dir = Path::new(&home).join(".claude").join("projects");
    let sessions_dir = Path::new(&home).join(".claude").join("sessions");

    let all = find_matching_sessions(&projects_dir, &sessions_dir, name);

    Ok(all.into_iter().filter(|s| is_process_alive(&s.pid)).collect())
}

/// Core search logic: scan `projects_dir` for JSONL files containing the
/// custom title, cross-reference `sessions_dir` to resolve PIDs.
/// Returns all matches regardless of whether the process is alive.
fn find_matching_sessions(
    projects_dir: &Path,
    sessions_dir: &Path,
    name: &str,
) -> Vec<LiveSession> {
    let title_needle = format!("\"customTitle\":\"{name}\"");
    let mut results = Vec::new();

    let project_entries = match fs::read_dir(projects_dir) {
        Ok(entries) => entries,
        Err(_) => return results,
    };

    for project_entry in project_entries.flatten() {
        let project_path = project_entry.path();
        if !project_path.is_dir() {
            continue;
        }

        let project_name = project_entry.file_name().to_string_lossy().to_string();

        let files = match fs::read_dir(&project_path) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for file_entry in files.flatten() {
            let file_path = file_entry.path();
            let file_name = file_entry.file_name().to_string_lossy().to_string();

            if !file_name.ends_with(".jsonl") {
                continue;
            }

            if !line_exists(&file_path, &title_needle) {
                continue;
            }

            let session_id = file_name.trim_end_matches(".jsonl");
            let session_needle = format!("\"sessionId\":\"{session_id}\"");

            if let Some(pid) = find_pid_for_session(sessions_dir, &session_needle) {
                results.push(LiveSession {
                    pid,
                    session_id: session_id.to_string(),
                    project: project_name.clone(),
                });
            }
        }
    }

    results
}

/// Scan a file line-by-line, returning true as soon as `needle` is found.
fn line_exists(path: &Path, needle: &str) -> bool {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    BufReader::new(file)
        .lines()
        .any(|line| line.map(|l| l.contains(needle)).unwrap_or(false))
}

/// Search `sessions_dir` for a JSON file containing `needle` and
/// return the PID extracted from its filename.
fn find_pid_for_session(sessions_dir: &Path, needle: &str) -> Option<String> {
    let entries = fs::read_dir(sessions_dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".json") {
            continue;
        }
        if line_exists(&path, needle) {
            return Some(name.trim_end_matches(".json").to_string());
        }
    }
    None
}

fn is_process_alive(pid: &str) -> bool {
    Command::new("kill")
        .args(["-0", pid])
        .stderr(std::process::Stdio::null())
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Create a temp directory with a unique name for each test.
    fn test_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("harnessx_session_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn cleanup(dir: &Path) {
        let _ = fs::remove_dir_all(dir);
    }

    /// Build a fake `~/.claude/projects/<project>/<session>.jsonl` file.
    fn write_project_session(
        projects_dir: &Path,
        project: &str,
        session_id: &str,
        custom_title: &str,
    ) {
        let dir = projects_dir.join(project);
        fs::create_dir_all(&dir).unwrap();
        let line = format!("{{\"customTitle\":\"{custom_title}\",\"other\":\"data\"}}");
        fs::write(dir.join(format!("{session_id}.jsonl")), line).unwrap();
    }

    /// Build a fake `~/.claude/sessions/<pid>.json` file.
    fn write_session_pid(sessions_dir: &Path, pid: &str, session_id: &str) {
        fs::create_dir_all(sessions_dir).unwrap();
        let content = format!("{{\"sessionId\":\"{session_id}\",\"type\":\"main\"}}");
        fs::write(sessions_dir.join(format!("{pid}.json")), content).unwrap();
    }

    // ── line_exists ──

    #[test]
    fn line_exists_finds_needle() {
        let dir = test_dir("line_exists_hit");
        let file = dir.join("test.jsonl");
        fs::write(&file, "line one\nline two has needle\nline three").unwrap();
        assert!(line_exists(&file, "needle"));
        cleanup(&dir);
    }

    #[test]
    fn line_exists_returns_false_on_miss() {
        let dir = test_dir("line_exists_miss");
        let file = dir.join("test.jsonl");
        fs::write(&file, "line one\nline two\nline three").unwrap();
        assert!(!line_exists(&file, "needle"));
        cleanup(&dir);
    }

    #[test]
    fn line_exists_returns_false_for_missing_file() {
        assert!(!line_exists(Path::new("/tmp/does_not_exist_xyz.jsonl"), "needle"));
    }

    // ── find_pid_for_session ──

    #[test]
    fn finds_pid_by_session_id() {
        let dir = test_dir("find_pid_hit");
        write_session_pid(&dir, "12345", "abc-def-123");
        write_session_pid(&dir, "67890", "other-session");

        let result = find_pid_for_session(&dir, "\"sessionId\":\"abc-def-123\"");
        assert_eq!(result, Some("12345".to_string()));
        cleanup(&dir);
    }

    #[test]
    fn returns_none_when_session_not_found() {
        let dir = test_dir("find_pid_miss");
        write_session_pid(&dir, "12345", "abc-def-123");

        let result = find_pid_for_session(&dir, "\"sessionId\":\"nonexistent\"");
        assert_eq!(result, None);
        cleanup(&dir);
    }

    #[test]
    fn ignores_non_json_files_in_sessions() {
        let dir = test_dir("find_pid_non_json");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("12345.txt"),
            "{\"sessionId\":\"abc-def-123\"}",
        )
        .unwrap();

        let result = find_pid_for_session(&dir, "\"sessionId\":\"abc-def-123\"");
        assert_eq!(result, None);
        cleanup(&dir);
    }

    #[test]
    fn returns_none_for_nonexistent_sessions_dir() {
        let result = find_pid_for_session(
            Path::new("/tmp/harnessx_no_such_dir_xyz"),
            "\"sessionId\":\"abc\"",
        );
        assert_eq!(result, None);
    }

    // ── find_matching_sessions ──

    #[test]
    fn finds_session_by_custom_title() {
        let dir = test_dir("match_title");
        let projects = dir.join("projects");
        let sessions = dir.join("sessions");

        write_project_session(&projects, "my-project", "sess-001", "my-cool-session");
        write_session_pid(&sessions, "99999", "sess-001");

        let results = find_matching_sessions(&projects, &sessions, "my-cool-session");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].session_id, "sess-001");
        assert_eq!(results[0].pid, "99999");
        assert_eq!(results[0].project, "my-project");
        cleanup(&dir);
    }

    #[test]
    fn returns_empty_when_title_not_found() {
        let dir = test_dir("match_no_title");
        let projects = dir.join("projects");
        let sessions = dir.join("sessions");

        write_project_session(&projects, "my-project", "sess-001", "other-title");
        write_session_pid(&sessions, "99999", "sess-001");

        let results = find_matching_sessions(&projects, &sessions, "nonexistent");
        assert!(results.is_empty());
        cleanup(&dir);
    }

    #[test]
    fn returns_empty_when_no_session_pid_mapping() {
        let dir = test_dir("match_no_pid");
        let projects = dir.join("projects");
        let sessions = dir.join("sessions");
        fs::create_dir_all(&sessions).unwrap();

        write_project_session(&projects, "my-project", "sess-001", "my-session");
        // No corresponding PID file in sessions/

        let results = find_matching_sessions(&projects, &sessions, "my-session");
        assert!(results.is_empty());
        cleanup(&dir);
    }

    #[test]
    fn finds_sessions_across_multiple_projects() {
        let dir = test_dir("match_multi_project");
        let projects = dir.join("projects");
        let sessions = dir.join("sessions");

        write_project_session(&projects, "project-a", "sess-aaa", "shared-name");
        write_project_session(&projects, "project-b", "sess-bbb", "shared-name");
        write_session_pid(&sessions, "111", "sess-aaa");
        write_session_pid(&sessions, "222", "sess-bbb");

        let mut results = find_matching_sessions(&projects, &sessions, "shared-name");
        results.sort_by(|a, b| a.pid.cmp(&b.pid));

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].pid, "111");
        assert_eq!(results[0].project, "project-a");
        assert_eq!(results[1].pid, "222");
        assert_eq!(results[1].project, "project-b");
        cleanup(&dir);
    }

    #[test]
    fn ignores_non_jsonl_files_in_project_dir() {
        let dir = test_dir("match_non_jsonl");
        let projects = dir.join("projects");
        let sessions = dir.join("sessions");

        let proj = projects.join("my-project");
        fs::create_dir_all(&proj).unwrap();
        // Write matching content but with .json extension (not .jsonl)
        fs::write(
            proj.join("sess-001.json"),
            "{\"customTitle\":\"target\"}",
        )
        .unwrap();
        write_session_pid(&sessions, "99999", "sess-001");

        let results = find_matching_sessions(&projects, &sessions, "target");
        assert!(results.is_empty());
        cleanup(&dir);
    }

    #[test]
    fn ignores_files_at_project_root_level() {
        let dir = test_dir("match_root_file");
        let projects = dir.join("projects");
        let sessions = dir.join("sessions");

        fs::create_dir_all(&projects).unwrap();
        // File directly in projects/ (not in a subdirectory)
        fs::write(
            projects.join("stray.jsonl"),
            "{\"customTitle\":\"target\"}",
        )
        .unwrap();
        write_session_pid(&sessions, "99999", "stray");

        let results = find_matching_sessions(&projects, &sessions, "target");
        assert!(results.is_empty());
        cleanup(&dir);
    }

    #[test]
    fn handles_nonexistent_projects_dir() {
        let sessions = Path::new("/tmp/harnessx_no_sessions_xyz");
        let projects = Path::new("/tmp/harnessx_no_projects_xyz");

        let results = find_matching_sessions(projects, sessions, "anything");
        assert!(results.is_empty());
    }

    #[test]
    fn only_matches_exact_title() {
        let dir = test_dir("match_exact");
        let projects = dir.join("projects");
        let sessions = dir.join("sessions");

        write_project_session(&projects, "proj", "sess-001", "my-session");
        write_project_session(&projects, "proj", "sess-002", "my-session-extended");
        write_session_pid(&sessions, "111", "sess-001");
        write_session_pid(&sessions, "222", "sess-002");

        let results = find_matching_sessions(&projects, &sessions, "my-session");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].session_id, "sess-001");
        cleanup(&dir);
    }

    // ── is_process_alive ──

    #[test]
    fn current_process_is_alive() {
        let pid = std::process::id().to_string();
        assert!(is_process_alive(&pid));
    }

    #[test]
    fn bogus_pid_is_not_alive() {
        assert!(!is_process_alive("999999999"));
    }

    #[test]
    fn non_numeric_pid_is_not_alive() {
        assert!(!is_process_alive("not-a-pid"));
    }
}
