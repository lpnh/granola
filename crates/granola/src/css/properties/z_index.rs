use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `z-index` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/z-index)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_z_index = CssZIndex::new().content("1");
///
/// assert_eq!(css_z_index.bake(), "z-index: 1;");
/// ```
///
/// # Askama template
///
/// ```askama
/// z-index: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ZIndexRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssZIndex<R: ZIndexRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: ZIndexRecipe> From<CssZIndex<R>> for CssDeclaration {
    fn from(css_z_index: CssZIndex<R>) -> Self {
        Self::new("z-index", css_z_index.bake_recipe().content)
    }
}
