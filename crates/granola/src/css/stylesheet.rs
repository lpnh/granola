#![allow(unused_qualifications)]

use askama::{FastWritable, Template};
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// A CSS style sheet.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let at_rule = CssAtRule::new()
///     .identifier("import")
///     .rule(r#"url("layout.css")"#);
///
/// let rule = CssRule::new()
///     .selectors_list("p")
///     .push_property(("color", "rebeccapurple"));
///
/// let css_stylesheet = CssStylesheet::new().push(at_rule).push(rule);
///
/// assert_eq!(
///     css_stylesheet.bake(),
///     r#"@import url("layout.css"); p { color: rebeccapurple; }"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ content | kirei }}
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[granola(format = css)]
#[recipe(name = StylesheetRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssStylesheet<R: StylesheetRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: StylesheetRecipe<Content = Bake>> CssStylesheet<R> {
    pub fn push(mut self, statement: impl FastWritable) -> Self {
        self.content.fold_in_ws(statement);
        self
    }

    pub fn push_rule(self, rule: impl Into<CssRule>) -> Self {
        self.push(rule.into())
    }
}

impl<R: RuleRecipe> From<CssRule<R>> for CssStylesheet {
    fn from(rule: CssRule<R>) -> Self {
        Self {
            content: rule.into(),
            ..Default::default()
        }
    }
}

impl<R: AtRuleRecipe> From<CssAtRule<R>> for CssStylesheet {
    fn from(at_rule: CssAtRule<R>) -> Self {
        Self {
            content: at_rule.into(),
            ..Default::default()
        }
    }
}

/// Shorthand for `CssStylesheet`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let at_rule = at_rule!("import", r#"url("layout.css")"#);
///
/// let rule = rule!("p", declaration!("color", "rebeccapurple"));
///
/// let css_stylesheet = stylesheet!(at_rule, rule);
///
/// assert_eq!(
///     css_stylesheet.bake(),
///     r#"@import url("layout.css"); p { color: rebeccapurple; }"#
/// );
#[macro_export]
macro_rules! stylesheet {
    () => {
        $crate::css::CssStylesheet::new()
    };
    ($content:expr $(,)?) => {
        $crate::css::CssStylesheet::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::css::CssStylesheet::new().content($crate::bake_ws![$first $(, $rest)*])
    };
}
