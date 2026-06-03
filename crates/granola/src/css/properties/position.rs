use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `position` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/position)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_position = CssPosition::new().content("relative");
///
/// assert_eq!(css_position.bake(), "position: relative;");
/// ```
///
/// # Askama template
///
/// ```askama
/// position: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PositionRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPosition<R: PositionRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: PositionRecipe> From<CssPosition<R>> for CssDeclaration {
    fn from(css_position: CssPosition<R>) -> Self {
        Self::new("position", css_position.bake_recipe().content)
    }
}

impl<R: PositionRecipe> From<CssPosition<R>> for CssDeclarationsBlock {
    fn from(css_position: CssPosition<R>) -> Self {
        Self::new().push(css_position)
    }
}
