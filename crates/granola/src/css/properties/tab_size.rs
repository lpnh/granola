use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `tab-size` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/tab-size)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_tab_size = CssTabSize::new().content("4");
///
/// assert_eq!(css_tab_size.bake(), "tab-size: 4;");
/// ```
///
/// # Askama template
///
/// ```askama
/// tab-size: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TabSizeRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTabSize<R: TabSizeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TabSizeRecipe> From<CssTabSize<R>> for CssDeclaration {
    fn from(css_tab_size: CssTabSize<R>) -> Self {
        Self::new("tab-size", css_tab_size.bake_recipe().content)
    }
}

impl<R: TabSizeRecipe> From<CssTabSize<R>> for CssDeclarationsBlock {
    fn from(css_tab_size: CssTabSize<R>) -> Self {
        Self::new().push(css_tab_size)
    }
}
