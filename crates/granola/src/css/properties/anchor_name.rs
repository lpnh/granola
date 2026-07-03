use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `anchor-name` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/anchor-name)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_anchor_name = CssAnchorName::new().content("--some-anchor");
///
/// assert_eq!(css_anchor_name.bake(), "anchor-name: --some-anchor;");
/// ```
///
/// # Askama template
///
/// ```askama
/// anchor-name: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = AnchorNameRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssAnchorName<R: AnchorNameRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: AnchorNameRecipe> From<CssAnchorName<R>> for CssDeclaration {
    fn from(css_anchor_name: CssAnchorName<R>) -> Self {
        Self::new("anchor-name", css_anchor_name.bake_recipe().content)
    }
}
