use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `text-decoration-skip-ink` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-decoration-skip-ink)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_decoration_skip_ink: CssTextDecorationSkipInk =
///     CssTextDecorationSkipInk::new("auto");
///
/// assert_eq!(
///     css_text_decoration_skip_ink.bake(),
///     "text-decoration-skip-ink: auto;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// text-decoration-skip-ink: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextDecorationSkipInkRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextDecorationSkipInk<R: TextDecorationSkipInkRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TextDecorationSkipInkRecipe> CssTextDecorationSkipInk<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TextDecorationSkipInkRecipe> From<CssTextDecorationSkipInk<R>> for CssDeclaration {
    fn from(css_text_decoration_skip_ink: CssTextDecorationSkipInk<R>) -> Self {
        Self::new(
            "text-decoration-skip-ink",
            css_text_decoration_skip_ink.value,
        )
    }
}

impl<R, B> From<CssTextDecorationSkipInk<R>> for CssDeclarationsBlock<B>
where
    R: TextDecorationSkipInkRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_text_decoration_skip_ink: CssTextDecorationSkipInk<R>) -> Self {
        Self::new().push(css_text_decoration_skip_ink)
    }
}
