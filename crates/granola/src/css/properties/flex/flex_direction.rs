use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `flex-direction` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/flex-direction)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_flex_direction = CssFlexDirection::new().content("row");
///
/// assert_eq!(css_flex_direction.bake(), "flex-direction: row;");
/// ```
///
/// # Askama template
///
/// ```askama
/// flex-direction: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FlexDirectionRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFlexDirection<R: FlexDirectionRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: FlexDirectionRecipe> From<CssFlexDirection<R>> for CssDeclaration {
    fn from(css_flex_direction: CssFlexDirection<R>) -> Self {
        Self::new("flex-direction", css_flex_direction.bake_recipe().content)
    }
}
