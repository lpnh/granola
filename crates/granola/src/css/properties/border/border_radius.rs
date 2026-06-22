use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `border-radius` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-radius)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_border_radius = CssBorderRadius::new().content("0.4em");
///
/// assert_eq!(css_border_radius.bake(), "border-radius: 0.4em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// border-radius: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BorderRadiusRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBorderRadius<R: BorderRadiusRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BorderRadiusRecipe> From<CssBorderRadius<R>> for CssDeclaration {
    fn from(css_border_radius: CssBorderRadius<R>) -> Self {
        Self::new("border-radius", css_border_radius.bake_recipe().content)
    }
}

impl<R: BorderRadiusRecipe> From<CssBorderRadius<R>> for CssDeclarationsBlock {
    fn from(css_border_radius: CssBorderRadius<R>) -> Self {
        Self::new().push(css_border_radius)
    }
}
