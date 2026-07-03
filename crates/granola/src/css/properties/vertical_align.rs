use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `vertical-align` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/vertical-align)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_vertical_align = CssVerticalAlign::new().content("baseline");
///
/// assert_eq!(css_vertical_align.bake(), "vertical-align: baseline;");
/// ```
///
/// # Askama template
///
/// ```askama
/// vertical-align: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = VerticalAlignRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssVerticalAlign<R: VerticalAlignRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: VerticalAlignRecipe> From<CssVerticalAlign<R>> for CssDeclaration {
    fn from(css_vertical_align: CssVerticalAlign<R>) -> Self {
        Self::new("vertical-align", css_vertical_align.bake_recipe().content)
    }
}
