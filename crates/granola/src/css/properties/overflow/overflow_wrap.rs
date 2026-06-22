use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `overflow-wrap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/overflow-wrap)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_overflow_wrap = CssOverflowWrap::new().content("break-word");
///
/// assert_eq!(css_overflow_wrap.bake(), "overflow-wrap: break-word;");
/// ```
///
/// # Askama template
///
/// ```askama
/// overflow-wrap: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OverflowWrapRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOverflowWrap<R: OverflowWrapRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: OverflowWrapRecipe> From<CssOverflowWrap<R>> for CssDeclaration {
    fn from(css_overflow_wrap: CssOverflowWrap<R>) -> Self {
        Self::new("overflow-wrap", css_overflow_wrap.bake_recipe().content)
    }
}

impl<R: OverflowWrapRecipe> From<CssOverflowWrap<R>> for CssDeclarationsBlock {
    fn from(css_overflow_wrap: CssOverflowWrap<R>) -> Self {
        Self::new().push(css_overflow_wrap)
    }
}
