use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `letter-spacing` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/letter-spacing)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_letter_spacing = CssLetterSpacing::new().content("inherit");
///
/// assert_eq!(css_letter_spacing.bake(), "letter-spacing: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// letter-spacing: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = LetterSpacingRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssLetterSpacing<R: LetterSpacingRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: LetterSpacingRecipe> From<CssLetterSpacing<R>> for CssDeclaration {
    fn from(css_letter_spacing: CssLetterSpacing<R>) -> Self {
        Self::new("letter-spacing", css_letter_spacing.bake_recipe().content)
    }
}

impl<R: LetterSpacingRecipe> From<CssLetterSpacing<R>> for CssDeclarationsBlock {
    fn from(css_letter_spacing: CssLetterSpacing<R>) -> Self {
        Self::new().push(css_letter_spacing)
    }
}
