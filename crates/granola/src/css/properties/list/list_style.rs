use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `list-style` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/list-style)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_list_style = CssListStyle::new().content("none");
///
/// assert_eq!(css_list_style.bake(), "list-style: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// list-style: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ListStyleRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssListStyle<R: ListStyleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: ListStyleRecipe> From<CssListStyle<R>> for CssDeclaration {
    fn from(css_list_style: CssListStyle<R>) -> Self {
        Self::new("list-style", css_list_style.bake_recipe().content)
    }
}
