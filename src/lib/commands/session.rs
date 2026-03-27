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

#[derive(Serialize)]
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

fn find_live_sessions(name: &str) -> ParserResult<Vec<LiveSession>> {
    let home = std::env::var("HOME").map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "HOME environment variable not set",
        )
    })?;

    let projects_dir = Path::new(&home).join(".claude").join("projects");
    let sessions_dir = Path::new(&home).join(".claude").join("sessions");

    let title_needle = format!("\"customTitle\":\"{name}\"");
    let mut results = Vec::new();

    let project_entries = match fs::read_dir(&projects_dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(results),
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

            if let Some(pid) = find_pid_for_session(&sessions_dir, &session_needle) {
                if is_process_alive(&pid) {
                    results.push(LiveSession {
                        pid,
                        session_id: session_id.to_string(),
                        project: project_name.clone(),
                    });
                }
            }
        }
    }

    Ok(results)
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

/// Search `~/.claude/sessions/` for a JSON file containing `needle` and
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
