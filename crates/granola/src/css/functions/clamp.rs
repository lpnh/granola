use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `clamp()` function.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/clamp)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_fn_clamp = CssFnClamp::new().min("1rem").val("2.5vw").max("2rem");
///
/// assert_eq!(css_fn_clamp.bake(), "clamp(1rem, 2.5vw, 2rem)");
/// ```
///
/// # Askama template
///
/// ```askama
/// clamp({{ min | kirei }}, {{ val | kirei }}, {{ max | kirei }})
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[recipe(name = FnClampRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFnClamp<R: FnClampRecipe = ()> {
    _recipe: PhantomData<R>,
    pub min: Bake,
    pub val: Bake,
    pub max: Bake,
}

impl<R: FnClampRecipe> CssFnClamp<R> {
    pub fn min(mut self, min: impl Into<Bake>) -> Self {
        self.min = min.into();
        self
    }

    pub fn val(mut self, val: impl Into<Bake>) -> Self {
        self.val = val.into();
        self
    }

    pub fn max(mut self, max: impl Into<Bake>) -> Self {
        self.max = max.into();
        self
    }
}
