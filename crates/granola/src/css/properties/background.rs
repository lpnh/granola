mod background_color;
pub use background_color::*;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `background` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/background)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_background = CssBackground::new().content("none");
///
/// assert_eq!(css_background.bake(), "background: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// background: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BackgroundRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBackground<R: BackgroundRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BackgroundRecipe> From<CssBackground<R>> for CssDeclaration {
    fn from(css_background: CssBackground<R>) -> Self {
        Self::new("background", css_background.bake_recipe().content)
    }
}

impl<R: BackgroundRecipe> From<CssBackground<R>> for CssDeclarationsBlock {
    fn from(css_background: CssBackground<R>) -> Self {
        Self::new().push(css_background)
    }
}
