use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `letter-spacing` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/letter-spacing)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_letter_spacing: CssLetterSpacing = CssLetterSpacing::new("inherit");
///
/// assert_eq!(css_letter_spacing.bake(), "letter-spacing: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// letter-spacing: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = LetterSpacingRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssLetterSpacing<R: LetterSpacingRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: LetterSpacingRecipe> CssLetterSpacing<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: LetterSpacingRecipe> From<CssLetterSpacing<R>> for CssDeclaration {
    fn from(css_letter_spacing: CssLetterSpacing<R>) -> Self {
        Self::new("letter-spacing", css_letter_spacing.value)
    }
}

impl<R, B> From<CssLetterSpacing<R>> for CssDeclarationsBlock<B>
where
    R: LetterSpacingRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_letter_spacing: CssLetterSpacing<R>) -> Self {
        Self::new().push(css_letter_spacing)
    }
}
