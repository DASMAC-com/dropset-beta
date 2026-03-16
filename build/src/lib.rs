/// An assembly directive to be emitted as a `.equ` in a constants file.
pub enum Constant {
    /// A memory offset that must fit in an i16.
    Offset {
        name: &'static str,
        doc: &'static str,
        value: i16,
    },
}
