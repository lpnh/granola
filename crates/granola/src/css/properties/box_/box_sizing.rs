use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `box-sizing` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/box-sizing)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_box_sizing = CssBoxSizing::new().content("border-box");
///
/// assert_eq!(css_box_sizing.bake(), "box-sizing: border-box;");
/// ```
///
/// # Askama template
///
/// ```askama
/// box-sizing: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BoxSizingRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBoxSizing<R: BoxSizingRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BoxSizingRecipe> From<CssBoxSizing<R>> for CssDeclaration {
    fn from(css_box_sizing: CssBoxSizing<R>) -> Self {
        Self::new("box-sizing", css_box_sizing.bake_recipe().content)
    }
}

impl<R: BoxSizingRecipe> From<CssBoxSizing<R>> for CssDeclarationsBlock {
    fn from(css_box_sizing: CssBoxSizing<R>) -> Self {
        Self::new().push(css_box_sizing)
    }
}
