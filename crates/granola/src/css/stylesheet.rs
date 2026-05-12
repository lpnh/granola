use askama::Template;

use crate::prelude::*;

/// A CSS style sheet.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_stylesheet = CssStyleSheet::new()
///     .push(CssRule::new("p", ("color", "rebeccapurple")));
///
/// assert_eq!(css_stylesheet.bake(),
/// "p {
///   color: rebeccapurple;
/// }");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_stylesheet: CssStyleSheet = [
///     CssRule::new("p", ("color", "rebeccapurple")),
///     CssRule::new(":root", ("--base-100", "oklch(93% 0.076 100.4)")),
/// ].into();
///
/// assert_eq!(css_stylesheet.bake(),
/// "p {
///   color: rebeccapurple;
/// }
///
/// :root {
///   --base-100: oklch(93% 0.076 100.4);
/// }");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_rule: CssRule = ("p", ("color", "rebeccapurple")).into();
///
/// let css_stylesheet: CssStyleSheet = css_rule.into();
///
/// assert_eq!(css_stylesheet.bake(),
/// "p {
///   color: rebeccapurple;
/// }");
/// ```
///
/// # Askama template
///
/// ```askama
/// {%- for r in rules -%}
/// {{- r -}}
/// {%- if !loop.last %}
///
/// {% endif -%}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssStyleSheet {
    pub rules: Vec<CssRule>,
}

impl CssStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, rule: impl Into<CssRule>) -> Self {
        self.rules.push(rule.into());
        self
    }
}

impl<R: Into<CssRule>, const N: usize> From<[R; N]> for CssStyleSheet {
    fn from(items: [R; N]) -> Self {
        Self {
            rules: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl<R: Into<CssRule>> From<Vec<R>> for CssStyleSheet {
    fn from(items: Vec<R>) -> Self {
        Self {
            rules: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CssRule> for CssStyleSheet {
    fn from(rule: CssRule) -> Self {
        Self { rules: vec![rule] }
    }
}

/// Shorthand for [`CssStyleSheet`].
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_stylesheet = stylesheet!(
///     rule!("p"; ("color", "rebeccapurple")),
///     rule!(":root"; ("--base-100", "oklch(93% 0.076 100.4)")),
/// );
///
/// assert_eq!(css_stylesheet.bake(),
/// "p {
///   color: rebeccapurple;
/// }
///
/// :root {
///   --base-100: oklch(93% 0.076 100.4);
/// }");
/// ```
#[macro_export]
macro_rules! stylesheet {
    ($rule: expr $(,)?) => {
        $crate::css::CssStyleSheet::new().push($rule)
    };
    ($first_rule: expr $(, $rest_rule: expr)+ $(,)?) => {
        $crate::css::CssStyleSheet::from([$first_rule $(, $rest_rule)*])
    };
}
