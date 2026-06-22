use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `font-family` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-family)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_family = CssFontFamily::new().content("inherit");
///
/// assert_eq!(css_font_family.bake(), "font-family: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// font-family: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontFamilyRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontFamily<R: FontFamilyRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: FontFamilyRecipe> From<CssFontFamily<R>> for CssDeclaration {
    fn from(css_font_family: CssFontFamily<R>) -> Self {
        Self::new("font-family", css_font_family.bake_recipe().content)
    }
}

impl<R: FontFamilyRecipe> From<CssFontFamily<R>> for CssDeclarationsBlock {
    fn from(css_font_family: CssFontFamily<R>) -> Self {
        Self::new().push(css_font_family)
    }
}
