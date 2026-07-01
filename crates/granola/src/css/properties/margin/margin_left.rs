use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `margin-left` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-left)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin_left = CssMarginLeft::new().content("1rem");
///
/// assert_eq!(css_margin_left.bake(), "margin-left: 1rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin-left: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginLeftRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMarginLeft<R: MarginLeftRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: MarginLeftRecipe> From<CssMarginLeft<R>> for CssDeclaration {
    fn from(css_margin_left: CssMarginLeft<R>) -> Self {
        Self::new("margin-left", css_margin_left.bake_recipe().content)
    }
}

impl<R: MarginLeftRecipe> From<CssMarginLeft<R>> for CssDeclarationsBlock {
    fn from(css_margin_left: CssMarginLeft<R>) -> Self {
        Self::new().push(css_margin_left)
    }
}
