use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `justify-content` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/justify-content)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_justify_content = CssJustifyContent::new().content("center");
///
/// assert_eq!(css_justify_content.bake(), "justify-content: center;");
/// ```
///
/// # Askama template
///
/// ```askama
/// justify-content: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = JustifyContentRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssJustifyContent<R: JustifyContentRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: JustifyContentRecipe> From<CssJustifyContent<R>> for CssDeclaration {
    fn from(css_justify_content: CssJustifyContent<R>) -> Self {
        Self::new("justify-content", css_justify_content.bake_recipe().content)
    }
}

impl<R: JustifyContentRecipe> From<CssJustifyContent<R>> for CssDeclarationsBlock {
    fn from(css_justify_content: CssJustifyContent<R>) -> Self {
        Self::new().push(css_justify_content)
    }
}
