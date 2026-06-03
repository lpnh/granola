use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `font-size` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-size)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_size = CssFontSize::new().content("0.875rem");
///
/// assert_eq!(css_font_size.bake(), "font-size: 0.875rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// font-size: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontSizeRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontSize<R: FontSizeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: FontSizeRecipe> From<CssFontSize<R>> for CssDeclaration {
    fn from(css_font_size: CssFontSize<R>) -> Self {
        Self::new("font-size", css_font_size.bake_recipe().content)
    }
}

impl<R: FontSizeRecipe> From<CssFontSize<R>> for CssDeclarationsBlock {
    fn from(css_font_size: CssFontSize<R>) -> Self {
        Self::new().push(css_font_size)
    }
}
