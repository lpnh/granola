use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `opacity` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/opacity)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_opacity: CssOpacity = CssOpacity::new("1");
///
/// assert_eq!(css_opacity.bake(), "opacity: 1;");
/// ```
///
/// # Askama template
///
/// ```askama
/// opacity: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OpacityRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOpacity<R: OpacityRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: OpacityRecipe> CssOpacity<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: OpacityRecipe> From<CssOpacity<R>> for CssDeclaration {
    fn from(css_opacity: CssOpacity<R>) -> Self {
        Self::new("opacity", css_opacity.value)
    }
}

impl<R, B> From<CssOpacity<R>> for CssDeclarationsBlock<B>
where
    R: OpacityRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_opacity: CssOpacity<R>) -> Self {
        Self::new().push(css_opacity)
    }
}
