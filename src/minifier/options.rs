//! Shared options passed to every minifier engine.

/// User selectable minification overrides.
#[derive(Debug, Clone, Default)]
pub struct MinifierOptions {
    /// Keep meaningful line breaks instead of joining everything. Engines that
    /// are naturally line based ignore this because they must keep lines.
    pub preserve_line_breaks: bool,
}

impl MinifierOptions {
    /// Options for a normal single-line run.
    pub fn single_line() -> Self {
        Self::default()
    }

    /// Options that keep line structure where an engine supports it.
    pub fn line_preserving() -> Self {
        Self {
            preserve_line_breaks: true,
        }
    }
}
