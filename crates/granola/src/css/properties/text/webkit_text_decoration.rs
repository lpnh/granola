use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `-webkit-text-decoration` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-decoration)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css = CssWebkitTextDecoration::new().content("none");
///
/// assert_eq!(css.bake(), "-webkit-text-decoration: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// -webkit-text-decoration: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = WebkitTextDecorationRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssWebkitTextDecoration<R: WebkitTextDecorationRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: WebkitTextDecorationRecipe<Content = Cow<'static, str>>> CssWebkitTextDecoration<R> {
    pub fn add_value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        let value = value.into();
        if value.is_empty() {
            return self;
        }
        self.content = if self.content.is_empty() {
            value
        } else {
            format!("{} {value}", self.content).into()
        };
        self
    }
}

impl<R: WebkitTextDecorationRecipe> From<CssWebkitTextDecoration<R>> for CssDeclaration {
    fn from(css_webkit_text_decoartion: CssWebkitTextDecoration<R>) -> Self {
        Self::new(
            "-webkit-text-decoration",
            css_webkit_text_decoartion.bake_recipe().content,
        )
    }
}

impl<R: WebkitTextDecorationRecipe> From<CssWebkitTextDecoration<R>> for CssDeclarationsBlock {
    fn from(css_webkit_text_decoartion: CssWebkitTextDecoration<R>) -> Self {
        Self::new().push(css_webkit_text_decoartion)
    }
}
