use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `box-sizing` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/box-sizing)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_box_sizing: CssBoxSizing = CssBoxSizing::new("border-box");
///
/// assert_eq!(css_box_sizing.bake(), "box-sizing: border-box;");
/// ```
///
/// # Askama template
///
/// ```askama
/// box-sizing: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BoxSizingRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBoxSizing<R: BoxSizingRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: BoxSizingRecipe> CssBoxSizing<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: BoxSizingRecipe> From<CssBoxSizing<R>> for CssDeclaration {
    fn from(css_box_sizing: CssBoxSizing<R>) -> Self {
        Self::new("box-sizing", css_box_sizing.value)
    }
}

impl<R, B> From<CssBoxSizing<R>> for CssDeclarationsBlock<B>
where
    R: BoxSizingRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_box_sizing: CssBoxSizing<R>) -> Self {
        Self::new().push(css_box_sizing)
    }
}
