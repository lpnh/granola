use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `white-space` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/white-space)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_white_space = CssWhiteSpace::new().content("nowrap");
///
/// assert_eq!(css_white_space.bake(), "white-space: nowrap;");
/// ```
///
/// # Askama template
///
/// ```askama
/// white-space: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = WhiteSpaceRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssWhiteSpace<R: WhiteSpaceRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: WhiteSpaceRecipe> From<CssWhiteSpace<R>> for CssDeclaration {
    fn from(css_white_space: CssWhiteSpace<R>) -> Self {
        Self::new("white-space", css_white_space.bake_recipe().content)
    }
}

impl<R: WhiteSpaceRecipe> From<CssWhiteSpace<R>> for CssDeclarationsBlock {
    fn from(css_white_space: CssWhiteSpace<R>) -> Self {
        Self::new().push(css_white_space)
    }
}
