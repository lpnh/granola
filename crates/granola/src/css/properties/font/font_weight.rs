use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `font-weight` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-weight)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_weight: CssFontWeight = CssFontWeight::new("500");
///
/// assert_eq!(css_font_weight.bake(), "font-weight: 500;");
/// ```
///
/// # Askama template
///
/// ```askama
/// font-weight: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontWeightRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontWeight<R: FontWeightRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: FontWeightRecipe> CssFontWeight<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: FontWeightRecipe> From<CssFontWeight<R>> for CssDeclaration {
    fn from(css_font_weight: CssFontWeight<R>) -> Self {
        Self::new("font-weight", css_font_weight.value)
    }
}

impl<R: FontWeightRecipe> From<CssFontWeight<R>> for CssPropertiesList {
    fn from(css_font_weight: CssFontWeight<R>) -> Self {
        Self {
            declarations: vec![css_font_weight.into()],
        }
    }
}
