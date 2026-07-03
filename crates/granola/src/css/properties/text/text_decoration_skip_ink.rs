use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

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
///     CssTextDecorationSkipInk::new().content("auto");
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
/// text-decoration-skip-ink: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextDecorationSkipInkRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextDecorationSkipInk<R: TextDecorationSkipInkRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TextDecorationSkipInkRecipe> From<CssTextDecorationSkipInk<R>> for CssDeclaration {
    fn from(css_text_decoration_skip_ink: CssTextDecorationSkipInk<R>) -> Self {
        Self::new(
            "text-decoration-skip-ink",
            css_text_decoration_skip_ink.bake_recipe().content,
        )
    }
}
