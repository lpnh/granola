use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `outline-offset` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-offset)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline_offset: CssOutlineOffset = CssOutlineOffset::new("2px");
///
/// assert_eq!(css_outline_offset.bake(), "outline-offset: 2px;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline-offset: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineOffsetRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutlineOffset<R: OutlineOffsetRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: OutlineOffsetRecipe> CssOutlineOffset<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: OutlineOffsetRecipe> From<CssOutlineOffset<R>> for CssDeclaration {
    fn from(css_outline_offset: CssOutlineOffset<R>) -> Self {
        Self::new("outline-offset", css_outline_offset.value)
    }
}

impl<R: OutlineOffsetRecipe> From<CssOutlineOffset<R>> for CssPropertiesList {
    fn from(css_outline_offset: CssOutlineOffset<R>) -> Self {
        Self {
            declarations: vec![css_outline_offset.into()],
        }
    }
}
