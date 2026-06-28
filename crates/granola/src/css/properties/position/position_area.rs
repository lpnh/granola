use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `position-area` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/position-area)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_position_area = CssPositionArea::new().content("top");
///
/// assert_eq!(css_position_area.bake(), "position-area: top;");
/// ```
///
/// # Askama template
///
/// ```askama
/// position-area: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PositionAreaRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPositionArea<R: PositionAreaRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: PositionAreaRecipe> From<CssPositionArea<R>> for CssDeclaration {
    fn from(css_position_area: CssPositionArea<R>) -> Self {
        Self::new("position-area", css_position_area.bake_recipe().content)
    }
}

impl<R: PositionAreaRecipe> From<CssPositionArea<R>> for CssDeclarationsBlock {
    fn from(css_position_area: CssPositionArea<R>) -> Self {
        Self::new().push(css_position_area)
    }
}
