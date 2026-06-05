use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

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
/// let block = bake_block![
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
///     at_rule.bake(),
///     "@keyframes slide-in {
///   from { transform: translateX(0%); }
///   to { transform: translateX(100%); }
/// }"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// @{{ identifier }} {{ rule }}
/// {%- if let Some(b) = block %} {
///   {{ b | indent(2) }}
/// }
/// {%- else %};
/// {%- endif %}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = AtRuleRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssAtRule<R: AtRuleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub identifier: Cow<'static, str>,
    pub rule: Cow<'static, str>,
    pub block: Option<Cow<'static, str>>,
}

impl<R: AtRuleRecipe> CssAtRule<R> {
    pub fn identifier(mut self, identifier: impl Into<Cow<'static, str>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    pub fn rule(mut self, rule: impl Into<Cow<'static, str>>) -> Self {
        self.rule = rule.into();
        self
    }

    pub fn block(mut self, block: impl Into<Cow<'static, str>>) -> Self {
        self.block = Some(block.into());
        self
    }
}

impl<I: Into<Cow<'static, str>>, R: Into<Cow<'static, str>>> From<(I, R)> for CssAtRule {
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
/// let block = bake_block![
///     "from { transform: translateX(0%); }",
///     "to { transform: translateX(100%); }",
/// ];
///
/// let at_rule = at_rule!("keyframes", "slide-in").block(block);
///
/// assert_eq!(
///     at_rule.bake(),
///     "@keyframes slide-in {
///   from { transform: translateX(0%); }
///   to { transform: translateX(100%); }
/// }"
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
    (@cookbook $($r:ty),+) => {
        $crate::css::CssAtRule::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
}
