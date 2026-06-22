use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `padding-inline-start` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding-inline-start)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_padding_inline_start = CssPaddingInlineStart::new().content("20px");
///
/// assert_eq!(
///     css_padding_inline_start.bake(),
///     "padding-inline-start: 20px;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// padding-inline-start: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PaddingInlineStartRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPaddingInlineStart<R: PaddingInlineStartRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: PaddingInlineStartRecipe> From<CssPaddingInlineStart<R>> for CssDeclaration {
    fn from(css_padding_inline_start: CssPaddingInlineStart<R>) -> Self {
        Self::new(
            "padding-inline-start",
            css_padding_inline_start.bake_recipe().content,
        )
    }
}

impl<R: PaddingInlineStartRecipe> From<CssPaddingInlineStart<R>> for CssDeclarationsBlock {
    fn from(css_padding_inline_start: CssPaddingInlineStart<R>) -> Self {
        Self::new().push(css_padding_inline_start)
    }
}
