// Catisen Browser - Text-Only / Minimalist Mode
// This module strips heavy JavaScript, CSS, and auto-playing media from web pages,
// leaving behind a clean, typography-focused layout.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReaderTheme {
    MentalityDark,
    TechManual,   // Based on Image 1 (White/Navy, clean sans-serif)
    TerminalBlue, // Based on Image 2 (Deep blue, white monospace, red accents)
}

pub struct ReaderMode {
    pub is_enabled: bool,
    pub current_theme: ReaderTheme,
}

impl ReaderMode {
    pub fn new() -> Self {
        ReaderMode { 
            is_enabled: false,
            current_theme: ReaderTheme::MentalityDark,
        }
    }

    /// Toggles the minimalist text-only mode
    pub fn toggle(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    /// Changes the active typography/color theme
    pub fn set_theme(&mut self, theme: ReaderTheme) {
        self.current_theme = theme;
    }

    /// Intercepts raw HTML before Servo renders it, stripping out the heavy bloat
    pub fn clean_html(&self, raw_html: &str) -> String {
        if !self.is_enabled {
            return raw_html.to_string(); 
        }

        let mut clean_doc = raw_html.to_string();

        clean_doc = Self::remove_html_tag(&clean_doc, "<script", "</script>");
        clean_doc = Self::remove_html_tag(&clean_doc, "<style", "</style>");
        clean_doc = Self::remove_html_tag(&clean_doc, "<video", "</video>");
        clean_doc = Self::remove_html_tag(&clean_doc, "<iframe", "</iframe>");

        let css = match self.current_theme {
            ReaderTheme::MentalityDark => "
                <style>
                    body { font-family: 'Fira Code', sans-serif; background-color: #121212; color: #E0E0E0; margin: 5% auto; max-width: 800px; line-height: 1.8; font-size: 18px; }
                    a { color: #f04747; text-decoration: none; }
                    img { max-width: 100%; border-radius: 8px; }
                </style>",
            ReaderTheme::TechManual => "
                <style>
                    body { font-family: 'Helvetica Neue', Arial, sans-serif; background-color: #F8F9FA; color: #333333; margin: 5% auto; max-width: 800px; line-height: 1.6; font-size: 18px; }
                    h1, h2, h3, h4, strong { color: #003366; font-weight: bold; } /* Deep Navy blue from the CE label */
                    a { color: #0055A4; text-decoration: underline; }
                    img { max-width: 100%; border: 1px solid #111; }
                </style>",
            ReaderTheme::TerminalBlue => "
                <style>
                    body { font-family: 'Courier New', monospace; background-color: #0000B3; color: #FFFFFF; margin: 5% auto; max-width: 800px; line-height: 1.5; font-size: 18px; text-transform: uppercase; }
                    h1, h2 { color: #FFFFFF; text-decoration: line-through decoration-color: #FF3333; decoration-thickness: 3px; } /* Red strikethrough from ticket */
                    a { color: #FFCC00; text-decoration: none; border-bottom: 2px solid #FFCC00; }
                    img { max-width: 100%; filter: grayscale(100%) contrast(1.2); }
                </style>"
        };

        clean_doc.insert_str(0, css);
        clean_doc
    }

    /// Helper function to rip out specific HTML blocks
    fn remove_html_tag(html: &str, start_tag: &str, end_tag: &str) -> String {
        let mut result = String::new();
        let mut current = html;

        while let Some(start_idx) = current.find(start_tag) {
            result.push_str(&current[..start_idx]);
            if let Some(end_offset) = current[start_idx..].find(end_tag) {
                let end_idx = start_idx + end_offset + end_tag.len();
                current = &current[end_idx..];
            } else {
                current = &current[start_idx + start_tag.len()..];
            }
        }
        result.push_str(current);
        result
    }
}
