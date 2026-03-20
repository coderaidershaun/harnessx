//! Embeds template directories into the binary and exposes them as a manifest.

use include_dir::{include_dir, Dir, File};

static AGENTS: Dir = include_dir!("$CARGO_MANIFEST_DIR/.claude/agents");
static HOOKS: Dir = include_dir!("$CARGO_MANIFEST_DIR/.claude/hooks");
static SKILLS: Dir = include_dir!("$CARGO_MANIFEST_DIR/.claude/skills");
static OBSIDIAN: Dir = include_dir!("$CARGO_MANIFEST_DIR/.obsidian");

/// A template file to be written during `harnessx init`, with a project-root-relative path.
pub struct TemplateFile {
    pub path: String,
    pub content: &'static [u8],
}

/// Host-specific; replaced with an empty JSON object so local layout isn't shipped.
const EMPTY_WORKSPACE: &str = ".obsidian/workspace.json";

const TEMPLATE_ROOTS: &[(&Dir, &str)] = &[
    (&AGENTS, ".claude/agents"),
    (&HOOKS, ".claude/hooks"),
    (&SKILLS, ".claude/skills"),
    (&OBSIDIAN, ".obsidian"),
];

/// Returns every template file that `harnessx init` should write.
pub fn manifest() -> Vec<TemplateFile> {
    TEMPLATE_ROOTS
        .iter()
        .flat_map(|(dir, prefix)| collect_recursive(dir, prefix))
        .collect()
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
