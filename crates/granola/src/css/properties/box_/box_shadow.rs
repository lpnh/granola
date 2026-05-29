use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `box-shadow` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/box-shadow)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_box_shadow: CssBoxShadow = CssBoxShadow::new("none");
///
/// assert_eq!(css_box_shadow.bake(), "box-shadow: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// box-shadow: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BoxShadowRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBoxShadow<R: BoxShadowRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: BoxShadowRecipe> CssBoxShadow<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: BoxShadowRecipe> From<CssBoxShadow<R>> for CssDeclaration {
    fn from(css_box_shadow: CssBoxShadow<R>) -> Self {
        Self::new("box-shadow", css_box_shadow.value)
    }
}

impl<R, B> From<CssBoxShadow<R>> for CssDeclarationsBlock<B>
where
    R: BoxShadowRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_box_shadow: CssBoxShadow<R>) -> Self {
        Self::new().push(css_box_shadow)
    }
}
