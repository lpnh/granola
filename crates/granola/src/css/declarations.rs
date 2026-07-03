use askama::Template;

use crate::prelude::*;

/// A CSS property and a CSS value pair.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Syntax/Introduction#css_declarations)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration = CssDeclaration::new("color", "rebeccapurple");
///
/// assert_eq!(css_declaration.bake(), "color: rebeccapurple;");
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ property }}: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssDeclaration {
    pub property: Bake,
    pub value: Bake,
}

impl CssDeclaration {
    pub fn new(property: impl Into<Bake>, value: impl Into<Bake>) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }

    pub fn important(mut self) -> Self {
        if !self.value.is_empty() {
            self.value = bake_ws!(self.value, "!important");
        }
        self
    }
}

impl<P: Into<Bake>, V: Into<Bake>> From<(P, V)> for CssDeclaration {
    fn from((property, value): (P, V)) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }
}

/// Shorthand for `CssDeclaration`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_declaration = declaration!("color", "rebeccapurple");
///
/// assert_eq!(css_declaration.bake(), "color: rebeccapurple;");
/// ```
#[macro_export]
macro_rules! declaration {
    ($prop:expr, $value:expr $(,)?) => {
        $crate::css::CssDeclaration::new($prop, $value)
    };
}

/// Creates [`Bake`] from a sequence of [`CssDeclaration`] values, separated
/// by a single space.
#[macro_export]
macro_rules! declarations_block {
    () => {
        $crate::oven::Bake::default()
    };
    ($($decl:expr),+ $(,)?) => {
        $crate::bake_ws![$($crate::css::CssDeclaration::from($decl)),+]
    };
}
