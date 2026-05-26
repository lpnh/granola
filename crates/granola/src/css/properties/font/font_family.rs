use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `font-family` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-family)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_family: CssFontFamily = CssFontFamily::new("inherit");
///
/// assert_eq!(css_font_family.bake(), "font-family: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// font-family: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontFamilyRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontFamily<R: FontFamilyRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: FontFamilyRecipe> CssFontFamily<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: FontFamilyRecipe> From<CssFontFamily<R>> for CssDeclaration {
    fn from(css_font_family: CssFontFamily<R>) -> Self {
        Self::new("font-family", css_font_family.value)
    }
}

impl<R: FontFamilyRecipe> From<CssFontFamily<R>> for CssPropertiesList {
    fn from(css_font_family: CssFontFamily<R>) -> Self {
        Self {
            declarations: vec![css_font_family.into()],
        }
    }
}
