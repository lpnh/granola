use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `border-collapse` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-collapse)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_border_collapse: CssBorderCollapse = CssBorderCollapse::new("collapse");
///
/// assert_eq!(css_border_collapse.bake(), "border-collapse: collapse;");
/// ```
///
/// # Askama template
///
/// ```askama
/// border-collapse: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BorderCollapseRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBorderCollapse<R: BorderCollapseRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: BorderCollapseRecipe> CssBorderCollapse<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: BorderCollapseRecipe> From<CssBorderCollapse<R>> for CssDeclaration {
    fn from(css_border_collapse: CssBorderCollapse<R>) -> Self {
        Self::new("border-collapse", css_border_collapse.value)
    }
}

impl<R, B> From<CssBorderCollapse<R>> for CssDeclarationsBlock<B>
where
    R: BorderCollapseRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_border_collapse: CssBorderCollapse<R>) -> Self {
        Self::new().push(css_border_collapse)
    }
}
