pub mod toml;
pub mod yaml;

/// Metadata for a value entry in an edit context
pub struct LineContext {
    pub value: String,
    pub line_number: usize,
}
