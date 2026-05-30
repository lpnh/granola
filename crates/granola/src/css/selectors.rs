#![allow(unused_qualifications)]

mod complex_selector;
pub use complex_selector::*;
mod compound_selector;
pub use compound_selector::*;
mod selectors_list;
pub use selectors_list::*;
mod simple_selector;
pub use simple_selector::*;

use askama::Template;
use std::borrow::Cow;

use crate::prelude::*;

/// A CSS combinator.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Combinators)
#[derive(Debug, Clone, Template, Granola)]
#[template(ext = "html")]
pub enum CssCombinator {
    /// The descendant combinator: " " (single space)
    #[template(source = " ")]
    Descendant,
    /// The child combinator: ">"
    #[template(source = " > ")]
    Child,
    /// The subsequent-sibling combinator: "~" (tilde)
    #[template(source = " ~ ")]
    SubsequentSibling,
    /// The next-sibling combinator: "+"
    #[template(source = " + ")]
    NextSibling,
    /// The column combinator: "||"
    #[template(source = " || ")]
    Column,
}

/// A CSS selector.
///
/// Either a [`SimpleSelector`], a [`CompoundSelector`], or a
/// [`ComplexSelector`].
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let selector: CssSelector = CssSelector::new("p");
///
/// assert_eq!(selector.bake(), "p");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let selector: CssSelector = "p".into();
///
/// assert_eq!(selector.bake(), "p");
/// ```
///
/// # Askama template
///
/// ```askama
/// {%- match self -%}
/// {%- when Self::Simple(s) %}{{- s -}}
/// {%- when Self::Compound(c) %}{{- c -}}
/// {%- when Self::Complex(cx) %}{{- cx -}}
/// {%- endmatch -%}
/// ```
#[derive(Debug, Clone, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub enum CssSelector {
    Simple(CssSimpleSelector),
    Compound(CssCompoundSelector),
    Complex(CssComplexSelector),
}

impl CssSelector {
    pub fn new(selector: impl Into<Cow<'static, str>>) -> Self {
        CssSelector::Simple(CssSimpleSelector::new(selector))
    }
}

impl<R: SimpleSelectorRecipe> From<CssSimpleSelector<R>> for CssSelector {
    fn from(s: CssSimpleSelector<R>) -> Self {
        CssSelector::Simple(s.bake_recipe())
    }
}

impl<R: CompoundSelectorRecipe> From<CssCompoundSelector<R>> for CssSelector {
    fn from(c: CssCompoundSelector<R>) -> Self {
        CssSelector::Compound(c.bake_recipe())
    }
}

impl<R: ComplexSelectorRecipe> From<CssComplexSelector<R>> for CssSelector {
    fn from(cx: CssComplexSelector<R>) -> Self {
        CssSelector::Complex(cx.bake_recipe())
    }
}

impl From<Cow<'static, str>> for CssSelector {
    fn from(s: Cow<'static, str>) -> Self {
        CssSelector::Simple(s.into())
    }
}

impl From<&'static str> for CssSelector {
    fn from(s: &'static str) -> Self {
        CssSelector::Simple(s.into())
    }
}

impl From<String> for CssSelector {
    fn from(s: String) -> Self {
        CssSelector::Simple(s.into())
    }
}

// Check whether a given simple selector is a type or universal selector.
//
// The function assumes the value is a simple, valid, non-compound selector.
pub(crate) fn is_type_or_universal(s: &str) -> bool {
    let mut chars = s.chars();

    let Some(first) = chars.next() else {
        return true;
    };

    match first {
        '*' => true,
        c if is_valid_ident_start(c) => true,
        '-' => matches!(chars.next(), Some(c) if is_valid_ident_start(c) || c == '-'),
        _ => false,
    }
}

fn is_valid_ident_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_') || c >= '\u{00A0}'
}

#[cfg(test)]
mod is_type_or_universal_tests {
    use super::is_type_or_universal;

    // return true if empty

    #[test]
    fn empty() {
        assert!(is_type_or_universal(""));
    }

    // return true for valid identifiers

    #[test]
    fn universal_selector() {
        assert!(is_type_or_universal("*"));
    }

    #[test]
    fn type_selector() {
        assert!(is_type_or_universal("p"));
    }

    #[test]
    fn vendor() {
        assert!(is_type_or_universal("-vendor"));
    }

    #[test]
    fn custom_property() {
        assert!(is_type_or_universal("--custom"));
    }

    #[test]
    fn alphanumeric_mix() {
        assert!(is_type_or_universal("nono79"));
    }

    #[test]
    fn alphanumeric_with_dash() {
        assert!(is_type_or_universal("ground-level"));
    }

    #[test]
    fn underscore_prefix() {
        assert!(is_type_or_universal("_internal"));
    }

    #[test]
    fn non_ascii_followed_by_numbers() {
        assert!(is_type_or_universal("🦀123"));
    }

    // return false for class, ID, attribute, pseudo-class

    #[test]
    fn class_selector() {
        assert!(!is_type_or_universal(".foo"));
    }

    #[test]
    fn id_selector() {
        assert!(!is_type_or_universal("#bar"));
    }

    #[test]
    fn attribute_selector() {
        assert!(!is_type_or_universal("[href]"));
    }

    #[test]
    fn pseudo_selector() {
        assert!(!is_type_or_universal(":hover"));
    }

    // it also return false for some invalid identifiers

    #[test]
    fn starts_with_digit() {
        assert!(!is_type_or_universal("34rem"));
    }

    #[test]
    fn hyphen_then_digit() {
        assert!(!is_type_or_universal("-12rad"));
    }

    #[test]
    fn single_quoted_string() {
        assert!(!is_type_or_universal("'scoobyDoo'"));
    }

    #[test]
    fn double_quoted_string() {
        assert!(!is_type_or_universal("\"scoobyDoo\""));
    }
}
