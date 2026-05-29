use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `bottom` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/bottom)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_bottom: CssBottom = CssBottom::new("-0.25em");
///
/// assert_eq!(css_bottom.bake(), "bottom: -0.25em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// bottom: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BottomRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBottom<R: BottomRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: BottomRecipe> CssBottom<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: BottomRecipe> From<CssBottom<R>> for CssDeclaration {
    fn from(css_bottom: CssBottom<R>) -> Self {
        Self::new("bottom", css_bottom.value)
    }
}

impl<R, B> From<CssBottom<R>> for CssDeclarationsBlock<B>
where
    R: BottomRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_bottom: CssBottom<R>) -> Self {
        Self::new().push(css_bottom)
    }
}
