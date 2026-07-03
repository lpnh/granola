use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `margin-top` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-top)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin_top = CssMarginTop::new().content("1rem");
///
/// assert_eq!(css_margin_top.bake(), "margin-top: 1rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin-top: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginTopRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMarginTop<R: MarginTopRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: MarginTopRecipe> From<CssMarginTop<R>> for CssDeclaration {
    fn from(css_margin_top: CssMarginTop<R>) -> Self {
        Self::new("margin-top", css_margin_top.bake_recipe().content)
    }
}
