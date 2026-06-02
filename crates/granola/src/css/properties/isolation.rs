use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `isolation` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/isolation)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_isolation: CssIsolation = CssIsolation::new("isolate");
///
/// assert_eq!(css_isolation.bake(), "isolation: isolate;");
/// ```
///
/// # Askama template
///
/// ```askama
/// isolation: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = IsolationRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssIsolation<R: IsolationRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: IsolationRecipe> CssIsolation<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: IsolationRecipe> From<CssIsolation<R>> for CssDeclaration {
    fn from(css_isolation: CssIsolation<R>) -> Self {
        Self::new("isolation", css_isolation.value)
    }
}

impl<R, B> From<CssIsolation<R>> for CssDeclarationsBlock<B>
where
    R: IsolationRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_isolation: CssIsolation<R>) -> Self {
        Self::new().push(css_isolation)
    }
}
