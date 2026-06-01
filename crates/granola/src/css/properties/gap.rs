use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `gap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/gap)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_gap: CssGap = CssGap::new("1rem");
///
/// assert_eq!(css_gap.bake(), "gap: 1rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// gap: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = GapRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssGap<R: GapRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: GapRecipe> CssGap<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: GapRecipe> From<CssGap<R>> for CssDeclaration {
    fn from(css_gap: CssGap<R>) -> Self {
        Self::new("gap", css_gap.value)
    }
}

impl<R, B> From<CssGap<R>> for CssDeclarationsBlock<B>
where
    R: GapRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_gap: CssGap<R>) -> Self {
        Self::new().push(css_gap)
    }
}
