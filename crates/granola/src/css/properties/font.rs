mod font_family;
pub use font_family::*;
mod font_feature_settings;
pub use font_feature_settings::*;
mod font_size;
pub use font_size::*;
mod font_variation_settings;
pub use font_variation_settings::*;
mod font_weight;
pub use font_weight::*;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `font` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font: CssFont = CssFont::new("inherit");
///
/// assert_eq!(css_font.bake(), "font: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// font: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFont<R: FontRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: FontRecipe> CssFont<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: FontRecipe> From<CssFont<R>> for CssDeclaration {
    fn from(css_font: CssFont<R>) -> Self {
        Self::new("font", css_font.value)
    }
}

impl<R, B> From<CssFont<R>> for CssDeclarationsBlock<B>
where
    R: FontRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_font: CssFont<R>) -> Self {
        Self::new().push(css_font)
    }
}
