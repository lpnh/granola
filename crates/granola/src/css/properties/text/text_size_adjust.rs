use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `text-size-adjust` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-size-adjust)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_size_adjust = CssTextSizeAdjust::new().content("none");
///
/// assert_eq!(css_text_size_adjust.bake(), "text-size-adjust: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-size-adjust: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextSizeAdjustRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextSizeAdjust<R: TextSizeAdjustRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TextSizeAdjustRecipe> From<CssTextSizeAdjust<R>> for CssDeclaration {
    fn from(css_text_size_adjust: CssTextSizeAdjust<R>) -> Self {
        Self::new(
            "text-size-adjust",
            css_text_size_adjust.bake_recipe().content,
        )
    }
}

impl<R: TextSizeAdjustRecipe> From<CssTextSizeAdjust<R>> for CssDeclarationsBlock {
    fn from(css_text_size_adjust: CssTextSizeAdjust<R>) -> Self {
        Self::new().push(css_text_size_adjust)
    }
}
