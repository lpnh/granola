use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `var()` function.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/var)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_fn_var = CssFnVar::new().custom_property("color-background");
///
/// assert_eq!(css_fn_var.bake(), "var(--color-background)");
/// ```
///
/// # Askama template
///
/// ```askama
/// var(--{{ custom_property | kirei -}}
///     {%- if let Some(f) = fallback %}, {{ f }}{% endif -%}
/// )
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[recipe(name = FnVarRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFnVar<R: FnVarRecipe = ()> {
    _recipe: PhantomData<R>,
    pub custom_property: Bake,
    pub fallback: Option<Bake>,
}

impl<R: FnVarRecipe> CssFnVar<R> {
    pub fn custom_property(mut self, custom_property: impl Into<Bake>) -> Self {
        self.custom_property = custom_property.into();
        self
    }

    pub fn fallback(mut self, fallback: impl Into<Bake>) -> Self {
        self.fallback = Some(fallback.into());
        self
    }
}
