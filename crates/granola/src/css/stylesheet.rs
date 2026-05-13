#![allow(unused_qualifications)]

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
/// let at_rule = CssAtRule::new("import", r#"url("layout.css")"#);
/// let rule = CssRule::new("p", ("color", "rebeccapurple"));
///
/// let css_stylesheet = CssStylesheet::new()
///     .push(at_rule)
///     .push(rule);
///
/// assert_eq!(css_stylesheet.bake(),
/// r#"@import url("layout.css");
///
/// p {
///   color: rebeccapurple;
/// }"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let at_rule: CssAtRule = ("import", r#"url("layout.css")"#).into();
///
/// let css_stylesheet: CssStylesheet = at_rule.into();
///
/// assert_eq!(css_stylesheet.bake(),
/// r#"@import url("layout.css");"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let rule: CssRule = ("p", ("color", "rebeccapurple")).into();
///
/// let css_stylesheet: CssStylesheet = rule.into();
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
/// {%- for s in statements -%}
/// {{- s -}}
/// {%- if !loop.last %}
///
/// {% endif -%}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssStylesheet {
    pub statements: Vec<CssStatement>,
}

impl CssStylesheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, rule: impl Into<CssStatement>) -> Self {
        self.statements.push(rule.into());
        self
    }
}

impl<S: Into<CssStatement>, const N: usize> From<[S; N]> for CssStylesheet {
    fn from(items: [S; N]) -> Self {
        Self {
            statements: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl<S: Into<CssStatement>> From<Vec<S>> for CssStylesheet {
    fn from(items: Vec<S>) -> Self {
        Self {
            statements: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CssRule> for CssStylesheet {
    fn from(rule: CssRule) -> Self {
        Self::new().push(rule)
    }
}

impl From<CssAtRule> for CssStylesheet {
    fn from(at_rule: CssAtRule) -> Self {
        Self::new().push(at_rule)
    }
}

/// The [`CssRule`] and [`CssAtRule`] CSS statements.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Syntax/Introduction#css_statements)
///
/// # Askama template
///
/// ```askama
/// {%- match self -%}
/// {%- when Self::Rule(r) %}{{ r }}
/// {%- when Self::AtRule(ar) %}{{ ar }}
/// {%- endmatch -%}
/// ```
#[derive(Debug, Clone, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub enum CssStatement {
    Rule(CssRule),
    AtRule(CssAtRule),
}

impl From<CssRule> for CssStatement {
    fn from(rule: CssRule) -> Self {
        Self::Rule(rule)
    }
}

impl From<CssAtRule> for CssStatement {
    fn from(at_rule: CssAtRule) -> Self {
        Self::AtRule(at_rule)
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
/// let rule = rule!("p"; ("color", "rebeccapurple"));
///
/// let css_stylesheet = stylesheet!(at_rule, rule);
///
/// assert_eq!(css_stylesheet.bake(),
/// r#"@import url("layout.css");
///
/// p {
///   color: rebeccapurple;
/// }"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let at_rule = at_rule!("import", r#"url("layout.css")"#);
///
/// let css_stylesheet = stylesheet!(at_rule);
///
/// assert_eq!(css_stylesheet.bake(),
/// r#"@import url("layout.css");"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let rule = rule!("p"; ("color", "rebeccapurple"));
///
/// let css_stylesheet = stylesheet!(rule);
///
/// assert_eq!(css_stylesheet.bake(),
/// "p {
///   color: rebeccapurple;
/// }");
/// ```
#[macro_export]
macro_rules! stylesheet {
    () => {
        $crate::css::CssStylesheet::new()
    };
    ($rule: expr $(,)?) => {
        $crate::css::CssStylesheet::from($rule)
    };
    ($first_rule: expr $(, $rest_rule: expr)+ $(,)?) => {
        $crate::css::CssStylesheet::from([
            $crate::css::CssStatement::from($first_rule)
            $(, $crate::css::CssStatement::from($rest_rule))*
        ])
    };
}
