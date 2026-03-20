//! Embeds template directories into the binary and exposes them as a manifest.

use include_dir::{include_dir, Dir, File};

static AGENTS: Dir = include_dir!("$CARGO_MANIFEST_DIR/.claude/agents");
static HOOKS: Dir = include_dir!("$CARGO_MANIFEST_DIR/.claude/hooks");
static SKILLS: Dir = include_dir!("$CARGO_MANIFEST_DIR/.claude/skills");
static OBSIDIAN: Dir = include_dir!("$CARGO_MANIFEST_DIR/.obsidian");
static DOCS: Dir = include_dir!("$CARGO_MANIFEST_DIR/docs");

const HARNESSX_README: &[u8] = b"# harnessx\n\nProject management data and documentation for this workspace.\n\nSee [docs/](docs/) for the full command reference.\n";

const ROOT_MD_CONTENT: &[u8] = b"# Project\n\nThis project is managed by harnessx.\n\nSee harnessx/docs/ for the full command reference.\n";

/// A template file to be written during `harnessx init`, with a project-root-relative path.
pub struct TemplateFile {
    pub path: String,
    pub content: &'static [u8],
}

/// The agent platform harnessx is being initialised for.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Agent {
    Claude,
    Cursor,
}

impl Agent {
    /// The config directory written to the project root (`.claude` or `.cursor`).
    pub fn config_dir(self) -> &'static str {
        match self {
            Agent::Claude => ".claude",
            Agent::Cursor => ".cursor",
        }
    }

    /// The root markdown file created in the project root.
    pub fn root_md(self) -> &'static str {
        match self {
            Agent::Claude => "CLAUDE.md",
            Agent::Cursor => "AGENTS.md",
        }
    }
}

/// Host-specific; replaced with an empty JSON object so local layout isn't shipped.
const EMPTY_WORKSPACE: &str = ".obsidian/workspace.json";

/// Returns every template file that `harnessx init` should write.
///
/// The `agent` determines whether the config directory is `.claude/` or `.cursor/`.
/// When `include_obsidian` is `true`, the `.obsidian/` vault configuration is included.
pub fn manifest(agent: Agent, include_obsidian: bool) -> Vec<TemplateFile> {
    let config_dir = agent.config_dir();

    let core_roots: Vec<(&Dir, String)> = vec![
        (&AGENTS, format!("{config_dir}/agents")),
        (&HOOKS, format!("{config_dir}/hooks")),
        (&SKILLS, format!("{config_dir}/skills")),
    ];

    let mut files: Vec<TemplateFile> = core_roots
        .iter()
        .flat_map(|(dir, prefix)| collect_recursive(dir, prefix))
        .collect();

    if include_obsidian {
        files.extend(collect_recursive(&OBSIDIAN, ".obsidian"));
    }

    files.extend(collect_recursive(&DOCS, "harnessx/docs"));

    files.push(TemplateFile {
        path: "harnessx/README.md".to_string(),
        content: HARNESSX_README,
    });

    files
}

/// The default content for the root markdown file (`CLAUDE.md` or `AGENTS.md`).
pub fn root_md_content() -> &'static [u8] {
    ROOT_MD_CONTENT
}

fn collect_recursive(dir: &'static Dir, prefix: &str) -> Vec<TemplateFile> {
    let files = dir.files().filter_map(|file| template_from(file, prefix));

    let subdirs = dir.dirs().flat_map(|sub| {
        let sub_name = sub.path().file_name().expect("embedded dir has name");
        collect_recursive(sub, &format!("{prefix}/{}", sub_name.to_string_lossy()))
    });

    files.chain(subdirs).collect()
}

fn template_from(file: &'static File, prefix: &str) -> Option<TemplateFile> {
    let name = file.path().file_name()?.to_string_lossy();
    if name.ends_with(".DS_Store") {
        return None;
    }

    let path = format!("{prefix}/{name}");
    let content = if path == EMPTY_WORKSPACE {
        b"{}".as_slice()
    } else {
        file.contents()
    };

    Some(TemplateFile { path, content })
}
