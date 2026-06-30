use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

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
#[recipe(name = DisplayRecipe, content = Cow<'static, str>, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssDisplay<R: DisplayRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: DisplayRecipe<Content = Cow<'static, str>>> CssDisplay<R> {
    pub fn add_value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        let value = value.into();
        if value.is_empty() {
            return self;
        }
        self.content = if self.content.is_empty() {
            value
        } else {
            format!("{} {value}", self.content).into()
        };
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
