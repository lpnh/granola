use askama::Template;
use std::marker::PhantomData;

use crate::prelude::*;

/// The CSS at-rule.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/At-rules)
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
/// assert_eq!(at_rule.bake(), r#"@import url("layout.css");"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let block = bake_ws![
///     "from { transform: translateX(0%); }",
///     "to { transform: translateX(100%); }",
/// ];
///
/// let at_rule = CssAtRule::new()
///     .identifier("keyframes")
///     .rule("slide-in")
///     .block(block);
///
/// assert_eq!(
///     at_rule.bake_pretty(),
///     "@keyframes slide-in {
///   from {
///     transform: translateX(0%);
///   }
///   to {
///     transform: translateX(100%);
///   }
/// }
/// "
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// @{{ identifier }} {{ rule }}
/// {%- if let Some(b) = block %} { {{ b }} }
/// {%- else %};
/// {%- endif -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[granola(format = css)]
#[recipe(name = AtRuleRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssAtRule<R: AtRuleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub identifier: Bake,
    pub rule: Bake,
    pub block: Option<Bake>,
}

impl<R: AtRuleRecipe> CssAtRule<R> {
    pub fn identifier(mut self, identifier: impl Into<Bake>) -> Self {
        self.identifier = identifier.into();
        self
    }

    pub fn rule(mut self, rule: impl Into<Bake>) -> Self {
        self.rule = rule.into();
        self
    }

    pub fn block(mut self, block: impl Into<Bake>) -> Self {
        self.block = Some(block.into());
        self
    }
}

impl<I: Into<Bake>, R: Into<Bake>> From<(I, R)> for CssAtRule {
    fn from((identifier, rule): (I, R)) -> Self {
        Self {
            identifier: identifier.into(),
            rule: rule.into(),
            ..Default::default()
        }
    }
}

/// Shorthand for `CssAtRule`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let at_rule = at_rule!("import", r#"url("layout.css")"#);
///
/// assert_eq!(at_rule.bake(), r#"@import url("layout.css");"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let block = bake_ws![
///     "from { transform: translateX(0%); }",
///     "to { transform: translateX(100%); }",
/// ];
///
/// let at_rule = at_rule!("keyframes", "slide-in").block(block);
///
/// assert_eq!(
///     at_rule.bake_pretty(),
///     "@keyframes slide-in {
///   from {
///     transform: translateX(0%);
///   }
///   to {
///     transform: translateX(100%);
///   }
/// }
/// "
/// );
/// ```
#[macro_export]
macro_rules! at_rule {
    () => {
        $crate::css::CssAtRule::new()
    };
    ($identifier:expr, $rule:expr $(,)?) => {
        $crate::css::CssAtRule::from(($identifier, $rule))
    };
}
