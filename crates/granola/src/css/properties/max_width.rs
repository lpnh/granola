use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `max-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/max-width)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_max_width = CssMaxWidth::new().content("100%");
///
/// assert_eq!(css_max_width.bake(), "max-width: 100%;");
/// ```
///
/// # Askama template
///
/// ```askama
/// max-width: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MaxWidthRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMaxWidth<R: MaxWidthRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: MaxWidthRecipe> From<CssMaxWidth<R>> for CssDeclaration {
    fn from(css_max_width: CssMaxWidth<R>) -> Self {
        Self::new("max-width", css_max_width.bake_recipe().content)
    }
}

impl<R: MaxWidthRecipe> From<CssMaxWidth<R>> for CssDeclarationsBlock {
    fn from(css_max_width: CssMaxWidth<R>) -> Self {
        Self::new().push(css_max_width)
    }
}
