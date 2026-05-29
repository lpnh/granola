use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, oven::BakeInto, prelude::*};

/// The `display` CSS property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/display)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_display: CssDisplay = CssDisplay::new("inline flex");
///
/// assert_eq!(css_display.bake(), "display: inline flex;");
/// ```
///
/// # Askama template
///
/// ```askama
/// display: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = DisplayRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssDisplay<R: DisplayRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: DisplayRecipe> From<CssDisplay<R>> for CssDeclaration {
    fn from(css_display: CssDisplay<R>) -> Self {
        Self::new("display", css_display.content.bake_into())
    }
}

impl<R, B> From<CssDisplay<R>> for CssDeclarationsBlock<B>
where
    R: DisplayRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_display: CssDisplay<R>) -> Self {
        Self::new().push(css_display)
    }
}
