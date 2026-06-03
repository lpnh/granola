// mod color_interpolation;
// mod color_interpolation_filters;
// mod color_scheme;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/color)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_color = CssColor::new().content("rebeccapurple");
///
/// assert_eq!(css_color.bake(), "color: rebeccapurple;");
/// ```
///
/// # Askama template
///
/// ```askama
/// color: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ColorRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssColor<R: ColorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: ColorRecipe> From<CssColor<R>> for CssDeclaration {
    fn from(css_color: CssColor<R>) -> Self {
        Self::new("color", css_color.bake_recipe().content)
    }
}

impl<R: ColorRecipe> From<CssColor<R>> for CssDeclarationsBlock {
    fn from(css_color: CssColor<R>) -> Self {
        Self::new().push(css_color)
    }
}
