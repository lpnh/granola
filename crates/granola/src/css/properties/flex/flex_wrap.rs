use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `flex-wrap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/flex-wrap)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_flex_wrap: CssFlexWrap = CssFlexWrap::new("nowrap");
///
/// assert_eq!(css_flex_wrap.bake(), "flex-wrap: nowrap;");
/// ```
///
/// # Askama template
///
/// ```askama
/// flex-wrap: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FlexWrapRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFlexWrap<R: FlexWrapRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: FlexWrapRecipe> CssFlexWrap<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: FlexWrapRecipe> From<CssFlexWrap<R>> for CssDeclaration {
    fn from(css_flex_wrap: CssFlexWrap<R>) -> Self {
        Self::new("flex-wrap", css_flex_wrap.value)
    }
}

impl<R, B> From<CssFlexWrap<R>> for CssDeclarationsBlock<B>
where
    R: FlexWrapRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_flex_wrap: CssFlexWrap<R>) -> Self {
        Self::new().push(css_flex_wrap)
    }
}
