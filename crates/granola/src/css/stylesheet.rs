#![allow(unused_qualifications)]

use askama::Template;
use std::marker::PhantomData;

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
/// let at_rule = CssAtRule::new()
///     .identifier("import")
///     .rule(r#"url("layout.css")"#);
///
/// let rule = CssRule::new()
///     .selectors_list("p")
///     .declarations_block(("color", "rebeccapurple"));
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
/// {%- for s in statements -%}
///     {{ s }}{% if !loop.last %} {% endif %}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[granola(format = css)]
#[recipe(name = StylesheetRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssStylesheet<R: StylesheetRecipe = ()> {
    _recipe: PhantomData<R>,
    pub statements: Vec<CssStatement>,
}

impl<R: StylesheetRecipe> CssStylesheet<R> {
    pub fn push(mut self, rule: impl Into<CssStatement>) -> Self {
        self.statements.push(rule.into());
        self
    }
}

impl<S: Into<CssStatement>, const N: usize> From<[S; N]> for CssStylesheet {
    fn from(items: [S; N]) -> Self {
        Self {
            statements: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl<S: Into<CssStatement>> From<Vec<S>> for CssStylesheet {
    fn from(items: Vec<S>) -> Self {
        Self {
            statements: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl<R: RuleRecipe> From<CssRule<R>> for CssStylesheet {
    fn from(rule: CssRule<R>) -> Self {
        Self::new().push(rule)
    }
}

impl<R: AtRuleRecipe> From<CssAtRule<R>> for CssStylesheet {
    fn from(at_rule: CssAtRule<R>) -> Self {
        Self::new().push(at_rule)
    }
}

/// The [`CssRule`] and [`CssAtRule`] CSS statements.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Syntax/Introduction#css_statements)
#[derive(Debug, Clone, Template, Granola)]
#[template(ext = "html", escape = "none")]
pub enum CssStatement {
    #[template(source = "{{ self.0 }}")]
    Rule(CssRule),
    #[template(source = "{{ self.0 }}")]
    AtRule(CssAtRule),
}

impl<R: RuleRecipe> From<CssRule<R>> for CssStatement {
    fn from(rule: CssRule<R>) -> Self {
        Self::Rule(rule.bake_recipe())
    }
}

impl<R: AtRuleRecipe> From<CssAtRule<R>> for CssStatement {
    fn from(at_rule: CssAtRule<R>) -> Self {
        Self::AtRule(at_rule.bake_recipe())
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
/// let rule = rule!("p", ("color", "rebeccapurple"));
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
    ($rule:expr $(,)?) => {
        $crate::css::CssStylesheet::from($rule)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::css::CssStylesheet::from([
            $crate::css::CssStatement::from($first)
            $(, $crate::css::CssStatement::from($rest))*
        ])
    };

    (@cookbook $r:ty $(,)?) => {
        $crate::css::CssStylesheet::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; @push $rule:expr $(,)?) => {
        $crate::css::CssStylesheet::<$r>::from_cookbook().push($rule)
    };
    (@cookbook $r:ty ; @push $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::css::CssStylesheet::<$r>::from_cookbook()
            .push($first)
            $(.push($rest))*
    };
}
