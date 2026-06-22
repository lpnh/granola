use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `appearance` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/appearance)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_appearance = CssAppearance::new().content("button");
///
/// assert_eq!(css_appearance.bake(), "appearance: button;");
/// ```
///
/// # Askama template
///
/// ```askama
/// appearance: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = AppearanceRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssAppearance<R: AppearanceRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: AppearanceRecipe> From<CssAppearance<R>> for CssDeclaration {
    fn from(css_appearance: CssAppearance<R>) -> Self {
        Self::new("appearance", css_appearance.bake_recipe().content)
    }
}

impl<R: AppearanceRecipe> From<CssAppearance<R>> for CssDeclarationsBlock {
    fn from(css_appearance: CssAppearance<R>) -> Self {
        Self::new().push(css_appearance)
    }
}
