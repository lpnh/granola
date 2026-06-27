use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `border-style` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-style)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_border_style = CssBorderStyle::new().content("1.2rem");
///
/// assert_eq!(css_border_style.bake(), "border-style: 1.2rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// border-style: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BorderRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBorderStyle<R: BorderRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BorderRecipe> From<CssBorderStyle<R>> for CssDeclaration {
    fn from(css_border_style: CssBorderStyle<R>) -> Self {
        Self::new("border-style", css_border_style.bake_recipe().content)
    }
}

impl<R: BorderRecipe> From<CssBorderStyle<R>> for CssDeclarationsBlock {
    fn from(css_border_style: CssBorderStyle<R>) -> Self {
        Self::new().push(css_border_style)
    }
}
