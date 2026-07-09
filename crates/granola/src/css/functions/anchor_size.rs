use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `anchor-size()` function.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/anchor-size)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_fn_anchor_size = CssFnAnchorSize::new().value("block");
///
/// assert_eq!(css_fn_anchor_size.bake(), "anchor-size(block)");
/// ```
///
/// # Askama template
///
/// ```askama
/// anchor-size({{ value | kirei }})
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[recipe(name = FnAnchorSizeRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFnAnchorSize<R: FnAnchorSizeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: FnAnchorSizeRecipe> CssFnAnchorSize<R> {
    pub fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.value = value.into();
        self
    }
}
