use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `content` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/content)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_content = CssContent::new().content(r#""this is new""#);
///
/// assert_eq!(css_content.bake(), r#"content: "this is new";"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// content: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ContentRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssContent<R: ContentRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: ContentRecipe> From<CssContent<R>> for CssDeclaration {
    fn from(css_content: CssContent<R>) -> Self {
        Self::new("content", css_content.bake_recipe().content)
    }
}

impl<R: ContentRecipe> From<CssContent<R>> for CssDeclarationsBlock {
    fn from(css_content: CssContent<R>) -> Self {
        Self::new().push(css_content)
    }
}
