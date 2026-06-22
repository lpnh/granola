/// Formats baked HTML with [`markup_fmt`] for readable output (e.g. snapshots,
/// debugging).
///
/// Renders as [`Language::Html`](markup_fmt::Language::Html) with default
/// options. Embedded CSS is routed through [`pretty_css`].
///
/// # Panics
///
/// Panics if [`markup_fmt`] returns an error. See [`markup_fmt::FormatError`].
pub fn pretty(html: &str) -> String {
    use markup_fmt::{Language, config::FormatOptions, format_text};

    format_text(
        html,
        Language::Html,
        &FormatOptions::default(),
        |code, hints| {
            Ok(if hints.ext == "css" {
                pretty_css(code).into()
            } else {
                code.into()
            })
        },
    )
    .expect("pretty: HTML formatting failed")
}

/// Formats baked CSS with [`malva`] for readable output (e.g. snapshots,
/// debugging).
///
/// Parses as [`Syntax::Css`](malva::Syntax::Css) with default options.
///
/// # Panics
///
/// Panics if [`malva`] returns an error.
pub fn pretty_css(css: &str) -> String {
    use malva::{Syntax, config::FormatOptions, format_text};

    format_text(css, Syntax::Css, &FormatOptions::default())
        .expect("pretty_css: CSS formatting failed")
}
