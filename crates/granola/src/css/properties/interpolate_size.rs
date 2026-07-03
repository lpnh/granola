use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `interpolate-size` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/interpolate-size)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_interpolate_size = CssInterpolateSize::new().content("allow-keywords");
///
/// assert_eq!(
///     css_interpolate_size.bake(),
///     "interpolate-size: allow-keywords;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// interpolate-size: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = InterpolateSizeRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssInterpolateSize<R: InterpolateSizeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: InterpolateSizeRecipe> From<CssInterpolateSize<R>> for CssDeclaration {
    fn from(css_interpolate_size: CssInterpolateSize<R>) -> Self {
        Self::new(
            "interpolate-size",
            css_interpolate_size.bake_recipe().content,
        )
    }
}
