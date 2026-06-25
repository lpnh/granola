use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

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
/// let css_fn_var = CssFnVar::new().content("--color-background");
///
/// assert_eq!(css_fn_var.bake(), "var(--color-background)");
/// ```
///
/// # Askama template
///
/// ```askama
/// var({{ content | kirei }})
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FnVarRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFnVar<R: FnVarRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}
