use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `text-wrap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-wrap)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_wrap = CssTextWrap::new().content("balance");
///
/// assert_eq!(css_text_wrap.bake(), "text-wrap: balance;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-wrap: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextWrapRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextWrap<R: TextWrapRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TextWrapRecipe> From<CssTextWrap<R>> for CssDeclaration {
    fn from(css_text_wrap: CssTextWrap<R>) -> Self {
        Self::new("text-wrap", css_text_wrap.bake_recipe().content)
    }
}
