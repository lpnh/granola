use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `color-mix()` function.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/color_value/color-mix)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_function_color_mix = CssFnColorMix::new().content("in oklch, red, blue");
///
/// assert_eq!(
///     css_function_color_mix.bake(),
///     "color-mix(in oklch, red, blue)"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// color-mix({{ content | kirei }})
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FnColorMixRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFnColorMix<R: FnColorMixRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}
