use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `line-height` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/line-height)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_line_height: CssLineHeight = CssLineHeight::new("1.25rem");
///
/// assert_eq!(css_line_height.bake(), "line-height: 1.25rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// line-height: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = LineHeightRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssLineHeight<R: LineHeightRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: LineHeightRecipe> CssLineHeight<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: LineHeightRecipe> From<CssLineHeight<R>> for CssDeclaration {
    fn from(css_line_height: CssLineHeight<R>) -> Self {
        Self::new("line-height", css_line_height.value)
    }
}

impl<R: LineHeightRecipe> From<CssLineHeight<R>> for CssDeclarationsBlock {
    fn from(css_line_height: CssLineHeight<R>) -> Self {
        Self {
            declarations: vec![css_line_height.into()],
        }
    }
}
