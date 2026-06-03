use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `background-color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/background-color)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_background_color = CssBackgroundColor::new().content("transparent");
///
/// assert_eq!(
///     css_background_color.bake(),
///     "background-color: transparent;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// background-color: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BackgroundColorRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBackgroundColor<R: BackgroundColorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BackgroundColorRecipe> From<CssBackgroundColor<R>> for CssDeclaration {
    fn from(css_background_color: CssBackgroundColor<R>) -> Self {
        Self::new(
            "background-color",
            css_background_color.bake_recipe().content,
        )
    }
}

impl<R: BackgroundColorRecipe> From<CssBackgroundColor<R>> for CssDeclarationsBlock {
    fn from(css_background_color: CssBackgroundColor<R>) -> Self {
        Self::new().push(css_background_color)
    }
}
