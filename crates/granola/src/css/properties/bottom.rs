use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `bottom` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/bottom)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_bottom = CssBottom::new().content("-0.25em");
///
/// assert_eq!(css_bottom.bake(), "bottom: -0.25em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// bottom: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BottomRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBottom<R: BottomRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BottomRecipe> From<CssBottom<R>> for CssDeclaration {
    fn from(css_bottom: CssBottom<R>) -> Self {
        Self::new("bottom", css_bottom.bake_recipe().content)
    }
}
