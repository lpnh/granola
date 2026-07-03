use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `border-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-width)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_border_width = CssBorderWidth::new().content("1.2rem");
///
/// assert_eq!(css_border_width.bake(), "border-width: 1.2rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// border-width: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BorderRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBorderWidth<R: BorderRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BorderRecipe> From<CssBorderWidth<R>> for CssDeclaration {
    fn from(css_border_width: CssBorderWidth<R>) -> Self {
        Self::new("border-width", css_border_width.bake_recipe().content)
    }
}
