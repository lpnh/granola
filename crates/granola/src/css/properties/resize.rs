use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `resize` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/resize)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_resize = CssResize::new().content("vertical");
///
/// assert_eq!(css_resize.bake(), "resize: vertical;");
/// ```
///
/// # Askama template
///
/// ```askama
/// resize: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ResizeRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssResize<R: ResizeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: ResizeRecipe> From<CssResize<R>> for CssDeclaration {
    fn from(css_resize: CssResize<R>) -> Self {
        Self::new("resize", css_resize.bake_recipe().content)
    }
}
