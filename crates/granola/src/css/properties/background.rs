mod background_color;
pub use background_color::*;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `background` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/background)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_background: CssBackground = CssBackground::new("none");
///
/// assert_eq!(css_background.bake(), "background: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// background: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BackgroundRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBackground<R: BackgroundRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: BackgroundRecipe> CssBackground<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: BackgroundRecipe> From<CssBackground<R>> for CssDeclaration {
    fn from(css_background: CssBackground<R>) -> Self {
        Self::new("background", css_background.value)
    }
}

impl<R, B> From<CssBackground<R>> for CssDeclarationsBlock<B>
where
    R: BackgroundRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_background: CssBackground<R>) -> Self {
        Self::new().push(css_background)
    }
}
