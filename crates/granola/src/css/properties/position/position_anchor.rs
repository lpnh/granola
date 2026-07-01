use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `position-anchor` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/position-anchor)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_position_anchor = CssPositionAnchor::new().content("--my-anchor");
///
/// assert_eq!(css_position_anchor.bake(), "position-anchor: --my-anchor;");
/// ```
///
/// # Askama template
///
/// ```askama
/// position-anchor: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PositionAnchorRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPositionAnchor<R: PositionAnchorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: PositionAnchorRecipe> From<CssPositionAnchor<R>> for CssDeclaration {
    fn from(css_position_anchor: CssPositionAnchor<R>) -> Self {
        Self::new("position-anchor", css_position_anchor.bake_recipe().content)
    }
}

impl<R: PositionAnchorRecipe> From<CssPositionAnchor<R>> for CssDeclarationsBlock {
    fn from(css_position_anchor: CssPositionAnchor<R>) -> Self {
        Self::new().push(css_position_anchor)
    }
}
