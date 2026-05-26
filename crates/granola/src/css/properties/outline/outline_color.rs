use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `outline-color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-color)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline_color: CssOutlineColor = CssOutlineColor::new("inherit");
///
/// assert_eq!(css_outline_color.bake(), "outline-color: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline-color: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineColorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutlineColor<R: OutlineColorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: OutlineColorRecipe> CssOutlineColor<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: OutlineColorRecipe> From<CssOutlineColor<R>> for CssDeclaration {
    fn from(css_outline_color: CssOutlineColor<R>) -> Self {
        Self::new("outline-color", css_outline_color.value)
    }
}

impl<R: OutlineColorRecipe> From<CssOutlineColor<R>> for CssPropertiesList {
    fn from(css_outline_color: CssOutlineColor<R>) -> Self {
        Self {
            declarations: vec![css_outline_color.into()],
        }
    }
}
