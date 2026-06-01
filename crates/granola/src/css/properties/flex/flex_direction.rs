use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `flex-direction` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/flex-direction)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_flex_direction: CssFlexDirection = CssFlexDirection::new("row");
///
/// assert_eq!(css_flex_direction.bake(), "flex-direction: row;");
/// ```
///
/// # Askama template
///
/// ```askama
/// flex-direction: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FlexDirectionRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFlexDirection<R: FlexDirectionRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: FlexDirectionRecipe> CssFlexDirection<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: FlexDirectionRecipe> From<CssFlexDirection<R>> for CssDeclaration {
    fn from(css_flex_direction: CssFlexDirection<R>) -> Self {
        Self::new("flex-direction", css_flex_direction.value)
    }
}

impl<R, B> From<CssFlexDirection<R>> for CssDeclarationsBlock<B>
where
    R: FlexDirectionRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_flex_direction: CssFlexDirection<R>) -> Self {
        Self::new().push(css_flex_direction)
    }
}
