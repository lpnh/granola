use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `height` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/height)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_height: CssHeight = CssHeight::new("auto");
///
/// assert_eq!(css_height.bake(), "height: auto;");
/// ```
///
/// # Askama template
///
/// ```askama
/// height: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = HeightRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssHeight<R: HeightRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: HeightRecipe> CssHeight<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: HeightRecipe> From<CssHeight<R>> for CssDeclaration {
    fn from(css_height: CssHeight<R>) -> Self {
        Self::new("height", css_height.value)
    }
}

impl<R: HeightRecipe> From<CssHeight<R>> for CssDeclarationsBlock {
    fn from(css_height: CssHeight<R>) -> Self {
        Self {
            declarations: vec![css_height.into()],
        }
    }
}
