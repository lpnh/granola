use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `position-try-fallbacks` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/position-try-fallbacks)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_position_try_fallbacks = CssPositionTryFallbacks::new().content("flip-block");
///
/// assert_eq!(
///     css_position_try_fallbacks.bake(),
///     "position-try-fallbacks: flip-block;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// position-try-fallbacks: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PositionTryFallbacksRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPositionTryFallbacks<R: PositionTryFallbacksRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: PositionTryFallbacksRecipe> From<CssPositionTryFallbacks<R>> for CssDeclaration {
    fn from(css_position_try_fallbacks: CssPositionTryFallbacks<R>) -> Self {
        Self::new(
            "position-try-fallbacks",
            css_position_try_fallbacks.bake_recipe().content,
        )
    }
}
