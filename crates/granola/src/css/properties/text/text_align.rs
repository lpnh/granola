use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `text-align` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-align)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_align = CssTextAlign::new().content("inherit");
///
/// assert_eq!(css_text_align.bake(), "text-align: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-align: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextAlignRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextAlign<R: TextAlignRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TextAlignRecipe> From<CssTextAlign<R>> for CssDeclaration {
    fn from(css_text_align: CssTextAlign<R>) -> Self {
        Self::new("text-align", css_text_align.bake_recipe().content)
    }
}

impl<R: TextAlignRecipe> From<CssTextAlign<R>> for CssDeclarationsBlock {
    fn from(css_text_align: CssTextAlign<R>) -> Self {
        Self::new().push(css_text_align)
    }
}
