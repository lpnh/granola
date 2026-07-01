use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `display` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/display)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_display = CssDisplay::new().content("inline flex");
///
/// assert_eq!(css_display.bake(), "display: inline flex;");
/// ```
///
/// # Askama template
///
/// ```askama
/// display: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = DisplayRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssDisplay<R: DisplayRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: DisplayRecipe<Content = Bake>> CssDisplay<R> {
    pub fn fold_in(mut self, value: impl Into<Bake>) -> Self {
        self.content.fold_in_ws(value);
        self
    }
}

impl<R: DisplayRecipe> From<CssDisplay<R>> for CssDeclaration {
    fn from(css_display: CssDisplay<R>) -> Self {
        Self::new("display", R::bake_content(css_display.content))
    }
}

impl<R: DisplayRecipe> From<CssDisplay<R>> for CssDeclarationsBlock {
    fn from(css_display: CssDisplay<R>) -> Self {
        Self::new().push(css_display)
    }
}
