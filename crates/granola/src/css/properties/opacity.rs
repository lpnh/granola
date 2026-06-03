use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `opacity` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/opacity)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_opacity = CssOpacity::new().content("1");
///
/// assert_eq!(css_opacity.bake(), "opacity: 1;");
/// ```
///
/// # Askama template
///
/// ```askama
/// opacity: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OpacityRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOpacity<R: OpacityRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: OpacityRecipe> From<CssOpacity<R>> for CssDeclaration {
    fn from(css_opacity: CssOpacity<R>) -> Self {
        Self::new("opacity", css_opacity.bake_recipe().content)
    }
}

impl<R: OpacityRecipe> From<CssOpacity<R>> for CssDeclarationsBlock {
    fn from(css_opacity: CssOpacity<R>) -> Self {
        Self::new().push(css_opacity)
    }
}
