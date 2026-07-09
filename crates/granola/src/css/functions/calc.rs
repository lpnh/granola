use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `calc()` function.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/calc)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_fn_calc = CssFnCalc::new().expression("100% - 1em");
///
/// assert_eq!(css_fn_calc.bake(), "calc(100% - 1em)");
/// ```
///
/// # Askama template
///
/// ```askama
/// calc({{ expression | kirei }})
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[recipe(name = FnCalcRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFnCalc<R: FnCalcRecipe = ()> {
    _recipe: PhantomData<R>,
    pub expression: Bake,
}

impl<R: FnCalcRecipe> CssFnCalc<R> {
    pub fn expression(mut self, expression: impl Into<Bake>) -> Self {
        self.expression = expression.into();
        self
    }

    pub fn add(mut self, first: impl Into<Bake>, second: impl Into<Bake>) -> Self {
        self.expression = bake_ws![first.into(), "+", second.into()];
        self
    }

    pub fn subtract(mut self, first: impl Into<Bake>, second: impl Into<Bake>) -> Self {
        self.expression = bake_ws![first.into(), "-", second.into()];
        self
    }

    pub fn multiply(mut self, first: impl Into<Bake>, second: impl Into<Bake>) -> Self {
        self.expression = bake_ws![first.into(), "*", second.into()];
        self
    }

    pub fn divide(mut self, first: impl Into<Bake>, second: impl Into<Bake>) -> Self {
        self.expression = bake_ws![first.into(), "/", second.into()];
        self
    }
}
