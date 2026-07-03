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
/// {{ statements | kirei }}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[granola(format = css)]
#[recipe(name = StylesheetRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssStylesheet<R: StylesheetRecipe = ()> {
    _recipe: PhantomData<R>,
    pub statements: Bake,
}

impl<R: StylesheetRecipe> CssStylesheet<R> {
    pub fn push(mut self, statement: impl FastWritable) -> Self {
        self.statements.fold_in_ws(statement);
        self
    }

    pub fn push_rule(mut self, rule: impl Into<CssRule>) -> Self {
        self = self.push(rule.into());
        self
    }
}

impl<R: RuleRecipe> From<CssRule<R>> for CssStylesheet {
    fn from(rule: CssRule<R>) -> Self {
        Self {
            statements: Bake::new(&rule),
            ..Default::default()
        }
    }
}

impl<R: AtRuleRecipe> From<CssAtRule<R>> for CssStylesheet {
    fn from(at_rule: CssAtRule<R>) -> Self {
        Self {
            statements: Bake::new(&at_rule),
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
    ($($statement:expr),+ $(,)?) => {{
        let mut css_stylesheet = $crate::css::CssStylesheet::new();
        css_stylesheet.statements = $crate::bake_ws![$($statement),+];
        css_stylesheet
    }};
}
