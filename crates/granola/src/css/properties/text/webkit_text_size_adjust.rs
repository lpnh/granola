use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `-webkit-text-size-adjust` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-size-adjust)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css = CssWebkitTextSizeAdjust::new().content("100%");
///
/// assert_eq!(css.bake(), "-webkit-text-size-adjust: 100%;");
/// ```
///
/// # Askama template
///
/// ```askama
/// -webkit-text-size-adjust: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = WebkitTextSizeAdjustRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssWebkitTextSizeAdjust<R: WebkitTextSizeAdjustRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: WebkitTextSizeAdjustRecipe> From<CssWebkitTextSizeAdjust<R>> for CssDeclaration {
    fn from(css_webkit_text_size_adjust: CssWebkitTextSizeAdjust<R>) -> Self {
        Self::new(
            "-webkit-text-size-adjust",
            css_webkit_text_size_adjust.bake_recipe().content,
        )
    }
}

impl<R: WebkitTextSizeAdjustRecipe> From<CssWebkitTextSizeAdjust<R>> for CssDeclarationsBlock {
    fn from(css_webkit_text_size_adjust: CssWebkitTextSizeAdjust<R>) -> Self {
        Self::new().push(css_webkit_text_size_adjust)
    }
}
