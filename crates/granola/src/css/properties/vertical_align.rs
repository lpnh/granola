use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `vertical-align` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/vertical-align)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_vertical_align: CssVerticalAlign = CssVerticalAlign::new("baseline");
///
/// assert_eq!(css_vertical_align.bake(), "vertical-align: baseline;");
/// ```
///
/// # Askama template
///
/// ```askama
/// vertical-align: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = VerticalAlignRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssVerticalAlign<R: VerticalAlignRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: VerticalAlignRecipe> CssVerticalAlign<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: VerticalAlignRecipe> From<CssVerticalAlign<R>> for CssDeclaration {
    fn from(css_vertical_align: CssVerticalAlign<R>) -> Self {
        Self::new("vertical-align", css_vertical_align.value)
    }
}

impl<R: VerticalAlignRecipe> From<CssVerticalAlign<R>> for CssDeclarationsBlock {
    fn from(css_vertical_align: CssVerticalAlign<R>) -> Self {
        Self {
            declarations: vec![css_vertical_align.into()],
        }
    }
}
