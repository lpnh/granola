use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `top` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/top)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_top = CssTop::new().content("-0.5em");
///
/// assert_eq!(css_top.bake(), "top: -0.5em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// top: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TopRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTop<R: TopRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TopRecipe> From<CssTop<R>> for CssDeclaration {
    fn from(css_top: CssTop<R>) -> Self {
        Self::new("top", css_top.bake_recipe().content)
    }
}

impl<R: TopRecipe> From<CssTop<R>> for CssDeclarationsBlock {
    fn from(css_top: CssTop<R>) -> Self {
        Self::new().push(css_top)
    }
}
