use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `border-color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-color)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_border_color = CssBorderColor::new().content("currentcolor");
///
/// assert_eq!(css_border_color.bake(), "border-color: currentcolor;");
/// ```
///
/// # Askama template
///
/// ```askama
/// border-color: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BorderColorRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBorderColor<R: BorderColorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BorderColorRecipe> From<CssBorderColor<R>> for CssDeclaration {
    fn from(css_border_color: CssBorderColor<R>) -> Self {
        Self::new("border-color", css_border_color.bake_recipe().content)
    }
}

impl<R: BorderColorRecipe> From<CssBorderColor<R>> for CssDeclarationsBlock {
    fn from(css_border_color: CssBorderColor<R>) -> Self {
        Self::new().push(css_border_color)
    }
}
