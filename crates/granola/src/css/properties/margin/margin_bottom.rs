use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `margin-bottom` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-bottom)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin_bottom = CssMarginBottom::new().content("1rem");
///
/// assert_eq!(css_margin_bottom.bake(), "margin-bottom: 1rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin-bottom: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginBottomRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMarginBottom<R: MarginBottomRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: MarginBottomRecipe> From<CssMarginBottom<R>> for CssDeclaration {
    fn from(css_margin_bottom: CssMarginBottom<R>) -> Self {
        Self::new("margin-bottom", css_margin_bottom.bake_recipe().content)
    }
}

impl<R: MarginBottomRecipe> From<CssMarginBottom<R>> for CssDeclarationsBlock {
    fn from(css_margin_bottom: CssMarginBottom<R>) -> Self {
        Self::new().push(css_margin_bottom)
    }
}
