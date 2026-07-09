use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `--*(...)` custom function.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Custom_functions_and_mixins)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_custom_function = CssCustomFunction::new()
///     .name("theme")
///     .content("--default-font-family, sans-serif");
///
/// assert_eq!(
///     css_custom_function.bake(),
///     "--theme(--default-font-family, sans-serif)"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// --{{ name }}({{ content | kirei }})
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[recipe(name = CustomFunctionRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssCustomFunction<R: CustomFunctionRecipe = ()> {
    _recipe: PhantomData<R>,
    pub name: Bake,
    pub content: R::Content,
}

impl<R: CustomFunctionRecipe> CssCustomFunction<R> {
    pub fn name(mut self, name: impl Into<Bake>) -> Self {
        self.name = name.into();
        self
    }
}
