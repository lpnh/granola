use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `isolation` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/isolation)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_isolation = CssIsolation::new().content("isolate");
///
/// assert_eq!(css_isolation.bake(), "isolation: isolate;");
/// ```
///
/// # Askama template
///
/// ```askama
/// isolation: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = IsolationRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssIsolation<R: IsolationRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: IsolationRecipe> From<CssIsolation<R>> for CssDeclaration {
    fn from(css_isolation: CssIsolation<R>) -> Self {
        Self::new("isolation", css_isolation.bake_recipe().content)
    }
}

impl<R: IsolationRecipe> From<CssIsolation<R>> for CssDeclarationsBlock {
    fn from(css_isolation: CssIsolation<R>) -> Self {
        Self::new().push(css_isolation)
    }
}
