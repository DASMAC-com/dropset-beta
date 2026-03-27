use std::collections::HashMap;
use std::path::Path;

/// Maximum line width for ASM output.
const MAX_LINE_WIDTH: usize = 75;

// region: types
/// An assembly comment for a generated directive.
pub struct Comment(pub &'static str);

/// A constant name for a generated directive.
pub struct Name(pub &'static str);

/// Common metadata shared by all assembly constant variants.
pub struct Header {
    pub name: Name,
    pub comment: Comment,
}

/// An assembly directive to be emitted via `.equ`.
pub enum Constant {
    /// An instruction offset that must fit in an i16.
    Offset { header: Header, value: i16 },
    /// An immediate value that must fit in an i32.
    Immediate { header: Header, value: i32 },
    /// A wide immediate value (i64), used with `lddw`.
    Wide { header: Header, value: i64 },
}

/// A named group of constants to be injected into an assembly file.
pub struct ConstantGroup {
    /// Injection target (e.g., "entrypoint" maps to program/src/dropset/entrypoint.s).
    pub target: &'static str,
    /// Optional group-level comment (from `///` doc on the constant group).
    pub comment: &'static str,
    /// The constants in this group.
    pub constants: &'static [Constant],
}
// endregion: types

impl Constant {
    /// The constant's name.
    fn name(&self) -> &'static str {
        match self {
            Constant::Offset { header, .. }
            | Constant::Immediate { header, .. }
            | Constant::Wide { header, .. } => header.name.0,
        }
    }

    /// Render this constant as an `.equ` directive.
    ///
    /// If the `.equ` and `# comment` fit on one line within `max_width`,
    /// they are placed on the same line. Otherwise the comment goes above.
    fn to_asm(&self, max_width: usize) -> String {
        let (name, comment, value_str) = match self {
            Constant::Offset { header, value } => {
                (header.name.0, header.comment.0, format!("{}", value))
            }
            Constant::Immediate { header, value } => {
                (header.name.0, header.comment.0, format!("{}", value))
            }
            Constant::Wide { header, value } => {
                (header.name.0, header.comment.0, format!("{}", value))
            }
        };

        let inline = format!(".equ {}, {} # {}", name, value_str, comment);
        if inline.len() <= max_width {
            inline
        } else {
            format!("# {}\n.equ {}, {}", comment, name, value_str)
        }
    }
}

/// Inject constant groups into assembly files.
///
/// Checks for duplicate constant names across all groups, merges groups
/// that share a target file, then for each target wipes all existing `.equ`
/// directives and injects the generated ones above the first label.
pub fn inject(asm_dir: &Path, groups: &[&ConstantGroup]) {
    // Check for duplicate names across all groups.
    let mut seen = HashMap::new();
    for group in groups {
        for constant in group.constants {
            let name = constant.name();
            if let Some(prev_target) = seen.insert(name, group.target) {
                panic!(
                    "duplicate constant name `{}` (in targets `{}` and `{}`)",
                    name, prev_target, group.target,
                );
            }
        }
    }

    // Collect groups by target file, preserving order.
    let mut targets: Vec<&str> = Vec::new();
    let mut by_target: HashMap<&str, Vec<&ConstantGroup>> = HashMap::new();
    for group in groups {
        if !by_target.contains_key(group.target) {
            targets.push(group.target);
        }
        by_target.entry(group.target).or_default().push(group);
    }

    // Validate all target files exist before writing any, so a single build
    // reports every missing file rather than failing on the first one.
    let missing: Vec<_> = targets
        .iter()
        .filter(|t| !asm_dir.join(format!("{}.s", t)).exists())
        .collect();
    if !missing.is_empty() {
        let list: Vec<String> = missing
            .iter()
            .map(|t| {
                format!(
                    "  {} (expected {})",
                    t,
                    asm_dir.join(format!("{}.s", t)).display()
                )
            })
            .collect();
        panic!("injection target(s) not found:\n{}", list.join("\n"),);
    }

    for target in targets {
        inject_target(asm_dir, target, &by_target[target]);
    }
}

/// `# ` prefix (2 chars) + dashes to fill the remaining width.
const SEPARATOR_BYTES: [u8; MAX_LINE_WIDTH] = {
    let mut buf = [b'-'; MAX_LINE_WIDTH];
    buf[0] = b'#';
    buf[1] = b' ';
    buf
};
const SEPARATOR: &str = unsafe { std::str::from_utf8_unchecked(&SEPARATOR_BYTES) };

fn render_group(group: &ConstantGroup) -> String {
    let directives: Vec<String> = group
        .constants
        .iter()
        .map(|c| c.to_asm(MAX_LINE_WIDTH))
        .collect();

    if group.comment.is_empty() {
        directives.join("\n")
    } else {
        format!(
            "# {}\n{}\n{}\n{}",
            group.comment,
            SEPARATOR,
            directives.join("\n"),
            SEPARATOR,
        )
    }
}

fn inject_target(asm_dir: &Path, target: &str, groups: &[&ConstantGroup]) {
    let file = asm_dir.join(format!("{}.s", target));
    let contents = std::fs::read_to_string(&file)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", file.display(), e));

    // Build the header from all groups.
    let rendered: Vec<String> = groups.iter().map(|g| render_group(g)).collect();
    let header = rendered.join("\n\n");

    // Wipe all .equ directives from the file first, then reconstruct.
    // This ensures stale constants never survive a re-injection regardless
    // of where they appear.
    let stripped: String = contents
        .lines()
        .filter(|line| !line.trim_start().starts_with(".equ "))
        .collect::<Vec<_>>()
        .join("\n");

    // Find the first label in the stripped content.
    let mut label_idx = None;
    for (i, line) in stripped.lines().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && trimmed.ends_with(':') {
            label_idx = Some(i);
            break;
        }
    }

    let output = if let Some(label_idx) = label_idx {
        // Reassemble: header + blank line + label onwards.
        let lines: Vec<&str> = stripped.lines().collect();
        let tail = lines[label_idx..].join("\n");
        format!("{}\n\n{}\n", header, tail)
    } else {
        // Constants-only file: just the header.
        format!("{}\n", header)
    };

    // Skip the write when content is unchanged. Cargo uses file modification
    // times to decide what to recompile, so rewriting an identical file still
    // invalidates the cache and forces a full rebuild of all downstream crates.
    // This is especially important in CI where the target/ directory is restored
    // from cache; without this check every run would recompile from scratch.
    if contents == output {
        return;
    }

    std::fs::write(&file, output)
        .unwrap_or_else(|e| panic!("failed to write {}: {}", file.display(), e));
}
