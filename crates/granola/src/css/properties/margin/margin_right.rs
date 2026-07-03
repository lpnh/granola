use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `margin-right` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-right)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin_right = CssMarginRight::new().content("1rem");
///
/// assert_eq!(css_margin_right.bake(), "margin-right: 1rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin-right: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginRightRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMarginRight<R: MarginRightRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: MarginRightRecipe> From<CssMarginRight<R>> for CssDeclaration {
    fn from(css_margin_right: CssMarginRight<R>) -> Self {
        Self::new("margin-right", css_margin_right.bake_recipe().content)
    }
}
