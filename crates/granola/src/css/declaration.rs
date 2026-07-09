use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// A CSS property and a CSS value pair.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Syntax/Introduction#css_declarations)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration = CssDeclaration::new()
///     .property("color")
///     .content("rebeccapurple");
///
/// assert_eq!(css_declaration.bake(), "color: rebeccapurple;");
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ property }}: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[recipe(name = DeclarationRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssDeclaration<R: DeclarationRecipe = ()> {
    _recipe: PhantomData<R>,
    pub property: Bake,
    pub content: R::Content,
}

impl<R: DeclarationRecipe> CssDeclaration<R> {
    pub fn property(mut self, property: impl Into<Bake>) -> Self {
        self.property = property.into();
        self
    }
}

impl<R: DeclarationRecipe<Content = Bake>> CssDeclaration<R> {
    /// Apply the `inherit` keyword to the property.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/inherit)
    pub fn inherit(mut self) -> Self {
        self.content = "inherit".into();
        self
    }

    /// Apply the `initial` keyword to the property.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/initial)
    pub fn initial(mut self) -> Self {
        self.content = "initial".into();
        self
    }

    /// Apply the `revert` keyword to the property.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/revert)
    pub fn revert(mut self) -> Self {
        self.content = "revert".into();
        self
    }

    /// Apply the `unset` keyword to the property.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/unset)
    pub fn unset(mut self) -> Self {
        self.content = "unset".into();
        self
    }

    /// Add the `!important` flag after the value, if any.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/important)
    pub fn important(mut self) -> Self {
        if !self.content.is_empty() {
            self.content.fold_in_ws("!important");
        }
        self
    }
}

impl<P: Into<Bake>, V: Into<Bake>> From<(P, V)> for CssDeclaration {
    fn from((property, value): (P, V)) -> Self {
        Self::new().property(property).content(value)
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
        $crate::css::CssDeclaration::from(($prop, $value))
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
