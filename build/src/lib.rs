/// Maximum line length for generated assembly files (for Vitepress code block).
const MAX_LINE_LENGTH: usize = 75;

/// An assembly comment for a generated directive.
///
/// Wraps a string that is guaranteed to fit within [`MAX_LINE_LENGTH`]
/// when rendered as `# {comment}`.
pub struct Comment(&'static str);

impl Comment {
    /// The prefix prepended to comment text in generated assembly.
    const PREFIX: &str = "# ";

    /// Creates a new `Comment`, panicking if the rendered line would
    /// exceed [`MAX_LINE_LENGTH`] characters.
    pub fn new(value: &'static str) -> Self {
        assert!(
            Self::PREFIX.len() + value.len() <= MAX_LINE_LENGTH,
            "comment exceeds maximum line length of {MAX_LINE_LENGTH}.",
        );
        Self(value)
    }
}

/// A constant name in CONSTANT_CASE (e.g., `INSN_TO_INSN_LEN_OFF`).
///
/// Only uppercase ASCII letters, digits, and underscores are allowed.
/// Must start with a letter and must not be empty.
pub struct Name(&'static str);

impl Name {
    /// Creates a new `Name`, panicking if the value is not valid
    /// CONSTANT_CASE.
    pub fn new(value: &'static str) -> Self {
        assert!(!value.is_empty(), "name must not be empty.");
        let first = value.as_bytes()[0];
        assert!(
            first.is_ascii_uppercase(),
            "name must start with an uppercase letter.",
        );
        assert!(
            value
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_'),
            "name must be CONSTANT_CASE (uppercase ASCII, digits, underscores).",
        );
        Self(value)
    }
}

/// Common metadata shared by all assembly constant variants.
pub struct Header {
    name: Name,
    comment: Comment,
}

impl Header {
    /// Creates a new `Header` with a validated name and comment.
    pub fn new(name: &'static str, comment: &'static str) -> Self {
        Self {
            name: Name::new(name),
            comment: Comment::new(comment),
        }
    }
}

/// An assembly directive to be emitted as a `.equ` in a constants file.
pub enum Constant {
    /// A memory offset that must fit in an i16.
    Offset { header: Header, value: i16 },
}
