use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/width)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_width = CssWidth::new().content("100%");
///
/// assert_eq!(css_width.bake(), "width: 100%;");
/// ```
///
/// # Askama template
///
/// ```askama
/// width: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = WidthRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssWidth<R: WidthRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: WidthRecipe> From<CssWidth<R>> for CssDeclaration {
    fn from(css_width: CssWidth<R>) -> Self {
        Self::new("width", css_width.bake_recipe().content)
    }
}

impl<R: WidthRecipe> From<CssWidth<R>> for CssDeclarationsBlock {
    fn from(css_width: CssWidth<R>) -> Self {
        Self::new().push(css_width)
    }
}
