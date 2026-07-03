use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `box-shadow` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/box-shadow)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_box_shadow = CssBoxShadow::new().content("none");
///
/// assert_eq!(css_box_shadow.bake(), "box-shadow: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// box-shadow: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BoxShadowRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBoxShadow<R: BoxShadowRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BoxShadowRecipe> From<CssBoxShadow<R>> for CssDeclaration {
    fn from(css_box_shadow: CssBoxShadow<R>) -> Self {
        Self::new("box-shadow", css_box_shadow.bake_recipe().content)
    }
}
