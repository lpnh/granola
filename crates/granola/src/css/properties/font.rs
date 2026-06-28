mod font_family;
pub use font_family::*;
mod font_feature_settings;
pub use font_feature_settings::*;
mod font_size;
pub use font_size::*;
mod font_style;
pub use font_style::*;
mod font_variation_settings;
pub use font_variation_settings::*;
mod font_weight;
pub use font_weight::*;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `font` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font = CssFont::new().content("inherit");
///
/// assert_eq!(css_font.bake(), "font: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// font: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFont<R: FontRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: FontRecipe> From<CssFont<R>> for CssDeclaration {
    fn from(css_font: CssFont<R>) -> Self {
        Self::new("font", css_font.bake_recipe().content)
    }
}

impl<R: FontRecipe> From<CssFont<R>> for CssDeclarationsBlock {
    fn from(css_font: CssFont<R>) -> Self {
        Self::new().push(css_font)
    }
}
