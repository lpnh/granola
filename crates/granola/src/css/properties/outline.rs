mod outline_color;
pub use outline_color::*;
mod outline_offset;
pub use outline_offset::*;
mod outline_style;
pub use outline_style::*;
mod outline_width;
pub use outline_width::*;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `outline` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline: CssOutline = CssOutline::new("auto");
///
/// assert_eq!(css_outline.bake(), "outline: auto;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutline<R: OutlineRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: OutlineRecipe> CssOutline<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: OutlineRecipe> From<CssOutline<R>> for CssDeclaration {
    fn from(css_outline: CssOutline<R>) -> Self {
        Self::new("outline", css_outline.value)
    }
}

impl<R, B> From<CssOutline<R>> for CssDeclarationsBlock<B>
where
    R: OutlineRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_outline: CssOutline<R>) -> Self {
        Self::new().push(css_outline)
    }
}
