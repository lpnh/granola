use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `line-height` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/line-height)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_line_height = CssLineHeight::new().content("1.25rem");
///
/// assert_eq!(css_line_height.bake(), "line-height: 1.25rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// line-height: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = LineHeightRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssLineHeight<R: LineHeightRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: LineHeightRecipe> From<CssLineHeight<R>> for CssDeclaration {
    fn from(css_line_height: CssLineHeight<R>) -> Self {
        Self::new("line-height", css_line_height.bake_recipe().content)
    }
}
