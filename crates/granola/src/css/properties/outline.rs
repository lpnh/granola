mod outline_color;
pub use outline_color::*;
mod outline_offset;
pub use outline_offset::*;
mod outline_style;
pub use outline_style::*;
mod outline_width;
pub use outline_width::*;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `outline` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline = CssOutline::new().content("auto");
///
/// assert_eq!(css_outline.bake(), "outline: auto;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutline<R: OutlineRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: OutlineRecipe> From<CssOutline<R>> for CssDeclaration {
    fn from(css_outline: CssOutline<R>) -> Self {
        Self::new("outline", css_outline.bake_recipe().content)
    }
}

impl<R: OutlineRecipe> From<CssOutline<R>> for CssDeclarationsBlock {
    fn from(css_outline: CssOutline<R>) -> Self {
        Self::new().push(css_outline)
    }
}
