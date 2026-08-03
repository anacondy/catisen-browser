// Catisen Browser - Text-Only / Reader Mode state.
//
// The live transformation is generated in browser/mod.rs and applied to the
// active WebView DOM. This module intentionally contains state only; the old
// raw-HTML/Servo helper was dead code and diverged from the real browser path.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReaderTheme {
    MentalityDark,
    TechManual,
    TerminalBlue,
}

pub struct ReaderMode {
    pub is_enabled: bool,
    pub current_theme: ReaderTheme,
}

impl Default for ReaderMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ReaderMode {
    pub fn new() -> Self {
        Self {
            is_enabled: false,
            current_theme: ReaderTheme::MentalityDark,
        }
    }

    pub fn toggle(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    #[allow(dead_code)]
    pub fn set_theme(&mut self, theme: ReaderTheme) {
        self.current_theme = theme;
    }
}
