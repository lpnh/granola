use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `position` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/position)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_position: CssPosition = CssPosition::new("relative");
///
/// assert_eq!(css_position.bake(), "position: relative;");
/// ```
///
/// # Askama template
///
/// ```askama
/// position: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PositionRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPosition<R: PositionRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: PositionRecipe> CssPosition<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: PositionRecipe> From<CssPosition<R>> for CssDeclaration {
    fn from(css_position: CssPosition<R>) -> Self {
        Self::new("position", css_position.value)
    }
}

impl<R, B> From<CssPosition<R>> for CssDeclarationsBlock<B>
where
    R: PositionRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_position: CssPosition<R>) -> Self {
        Self::new().push(css_position)
    }
}
