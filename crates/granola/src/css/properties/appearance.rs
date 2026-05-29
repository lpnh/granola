use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `appearance` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/appearance)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_appearance: CssAppearance = CssAppearance::new("button");
///
/// assert_eq!(css_appearance.bake(), "appearance: button;");
/// ```
///
/// # Askama template
///
/// ```askama
/// appearance: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = AppearanceRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssAppearance<R: AppearanceRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: AppearanceRecipe> CssAppearance<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: AppearanceRecipe> From<CssAppearance<R>> for CssDeclaration {
    fn from(css_appearance: CssAppearance<R>) -> Self {
        Self::new("appearance", css_appearance.value)
    }
}

impl<R, B> From<CssAppearance<R>> for CssDeclarationsBlock<B>
where
    R: AppearanceRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_appearance: CssAppearance<R>) -> Self {
        Self::new().push(css_appearance)
    }
}
