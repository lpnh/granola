mod margin_inline_end;
pub use margin_inline_end::*;

// margin_block
// margin_block_end
// margin_block_start
// margin_bottom
// margin_inline
// margin_inline_start
// margin_left
// margin_right
// margin_top

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `margin` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin: CssMargin = CssMargin::new("0");
///
/// assert_eq!(css_margin.bake(), "margin: 0;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMargin<R: MarginRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: MarginRecipe> CssMargin<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: MarginRecipe> From<CssMargin<R>> for CssDeclaration {
    fn from(css_margin: CssMargin<R>) -> Self {
        Self::new("margin", css_margin.value)
    }
}

impl<R, B> From<CssMargin<R>> for CssDeclarationsBlock<B>
where
    R: MarginRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_margin: CssMargin<R>) -> Self {
        Self::new().push(css_margin)
    }
}
