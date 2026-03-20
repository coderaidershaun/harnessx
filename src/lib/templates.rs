//! Embeds template directories into the binary and exposes them as a manifest.

use include_dir::{include_dir, Dir, File};

static AGENTS: Dir = include_dir!("$CARGO_MANIFEST_DIR/.claude/agents");
static HOOKS: Dir = include_dir!("$CARGO_MANIFEST_DIR/.claude/hooks");
static SKILLS: Dir = include_dir!("$CARGO_MANIFEST_DIR/.claude/skills");
static OBSIDIAN: Dir = include_dir!("$CARGO_MANIFEST_DIR/.obsidian");
static DOCS: Dir = include_dir!("$CARGO_MANIFEST_DIR/docs");

const HARNESSX_README: &[u8] = b"# harnessx\n\nProject management data and documentation for this workspace.\n\nSee [docs/](docs/) for the full command reference.\n";

/// A template file to be written during `harnessx init`, with a project-root-relative path.
pub struct TemplateFile {
    pub path: String,
    pub content: &'static [u8],
}

/// Host-specific; replaced with an empty JSON object so local layout isn't shipped.
const EMPTY_WORKSPACE: &str = ".obsidian/workspace.json";

const CORE_ROOTS: &[(&Dir, &str)] = &[
    (&AGENTS, ".claude/agents"),
    (&HOOKS, ".claude/hooks"),
    (&SKILLS, ".claude/skills"),
];

const OBSIDIAN_ROOT: (&Dir, &str) = (&OBSIDIAN, ".obsidian");

/// Returns every template file that `harnessx init` should write.
///
/// When `include_obsidian` is `true`, the `.obsidian/` vault configuration is
/// included in the manifest; otherwise only the core harness files are emitted.
pub fn manifest(include_obsidian: bool) -> Vec<TemplateFile> {
    let roots: Vec<(&Dir, &str)> = if include_obsidian {
        let mut r: Vec<_> = CORE_ROOTS.to_vec();
        r.push(OBSIDIAN_ROOT);
        r
    } else {
        CORE_ROOTS.to_vec()
    };

    let mut files: Vec<TemplateFile> = roots
        .iter()
        .flat_map(|(dir, prefix)| collect_recursive(dir, prefix))
        .collect();

    files.extend(collect_recursive(&DOCS, "harnessx/docs"));

    files.push(TemplateFile {
        path: "harnessx/README.md".to_string(),
        content: HARNESSX_README,
    });

    files
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
