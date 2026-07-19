//! Syntax highlighting initialization.
//!
//! Provides lazily-initialized `Syntect` instances for code highlighting.
//! Dark themes (MullNight, TokyoNight) share `mull-night.tmTheme`;
//! MullDay uses `mull-day.tmTheme` with deepened colors for light backgrounds.

use std::sync::OnceLock;

pub use mull_markdown::Syntect;

use crate::theme::ThemeKind;

static SYNTECT_MULLNIGHT: OnceLock<Syntect> = OnceLock::new();
static SYNTECT_TOKYONIGHT: OnceLock<Syntect> = OnceLock::new();
static SYNTECT_MULLDAY: OnceLock<Syntect> = OnceLock::new();

/// Convert syntect style to ratatui foreground-only style, quantized for terminal color support.
pub fn syntect_to_ratatui_fg(style: syntect::highlighting::Style) -> ratatui::style::Style {
    let fg = crate::theme::quantize(ratatui::style::Color::Rgb(
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
    ));
    let mut out = ratatui::style::Style::default().fg(fg);
    use syntect::highlighting::FontStyle;
    if style.font_style.contains(FontStyle::BOLD) {
        out = out.add_modifier(ratatui::style::Modifier::BOLD);
    }
    if style.font_style.contains(FontStyle::ITALIC) {
        out = out.add_modifier(ratatui::style::Modifier::ITALIC);
    }
    if style.font_style.contains(FontStyle::UNDERLINE) {
        out = out.add_modifier(ratatui::style::Modifier::UNDERLINED);
    }
    out
}

/// Highlight a single line of source, falling back to plain text style.
pub fn highlight_line(
    text: &str,
    highlighter: &mut Option<syntect::easy::HighlightLines<'_>>,
    syntect: &Syntect,
    fallback: ratatui::style::Style,
) -> Vec<ratatui::text::Span<'static>> {
    if let Some(hl) = highlighter.as_mut()
        && let Ok(ranges) = hl.highlight_line(&format!("{text}\n"), &syntect.syntax_set)
    {
        let mut spans = Vec::new();
        for (style, segment) in ranges {
            let mut s = segment.to_owned();
            while s.ends_with('\n') || s.ends_with('\r') {
                s.pop();
            }
            if s.is_empty() {
                continue;
            }
            spans.push(ratatui::text::Span::styled(s, syntect_to_ratatui_fg(style)));
        }
        if !spans.is_empty() {
            return spans;
        }
    }
    vec![ratatui::text::Span::styled(text.to_string(), fallback)]
}

/// Returns the syntect instance matching the active theme.
pub fn get_syntect() -> &'static Syntect {
    match crate::theme::Theme::current_kind() {
        ThemeKind::MullNight
        | ThemeKind::RosePineMoon
        | ThemeKind::OscuraMidnight
        | ThemeKind::Auto => SYNTECT_MULLNIGHT
            .get_or_init(|| Syntect::new(include_bytes!("../assets/mull-night.tmTheme"))),
        ThemeKind::TokyoNight => SYNTECT_TOKYONIGHT
            .get_or_init(|| Syntect::new(include_bytes!("../assets/tokyo-night.tmTheme"))),
        ThemeKind::MullDay => SYNTECT_MULLDAY
            .get_or_init(|| Syntect::new(include_bytes!("../assets/mull-day.tmTheme"))),
    }
}
