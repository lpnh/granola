use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `anchor-scope` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/anchor-scope)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_anchor_scope = CssAnchorScope::new().content("all");
///
/// assert_eq!(css_anchor_scope.bake(), "anchor-scope: all;");
/// ```
///
/// # Askama template
///
/// ```askama
/// anchor-scope: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = AnchorScopeRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssAnchorScope<R: AnchorScopeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: AnchorScopeRecipe> From<CssAnchorScope<R>> for CssDeclaration {
    fn from(css_anchor_scope: CssAnchorScope<R>) -> Self {
        Self::new("anchor-scope", css_anchor_scope.bake_recipe().content)
    }
}
