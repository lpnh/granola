use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `font-weight` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-weight)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_weight = CssFontWeight::new().content("500");
///
/// assert_eq!(css_font_weight.bake(), "font-weight: 500;");
/// ```
///
/// # Askama template
///
/// ```askama
/// font-weight: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontWeightRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontWeight<R: FontWeightRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: FontWeightRecipe> From<CssFontWeight<R>> for CssDeclaration {
    fn from(css_font_weight: CssFontWeight<R>) -> Self {
        Self::new("font-weight", css_font_weight.bake_recipe().content)
    }
}

impl<R: FontWeightRecipe> From<CssFontWeight<R>> for CssDeclarationsBlock {
    fn from(css_font_weight: CssFontWeight<R>) -> Self {
        Self::new().push(css_font_weight)
    }
}
