use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `border-collapse` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-collapse)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_border_collapse = CssBorderCollapse::new().content("collapse");
///
/// assert_eq!(css_border_collapse.bake(), "border-collapse: collapse;");
/// ```
///
/// # Askama template
///
/// ```askama
/// border-collapse: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BorderCollapseRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBorderCollapse<R: BorderCollapseRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BorderCollapseRecipe> From<CssBorderCollapse<R>> for CssDeclaration {
    fn from(css_border_collapse: CssBorderCollapse<R>) -> Self {
        Self::new("border-collapse", css_border_collapse.bake_recipe().content)
    }
}

impl<R: BorderCollapseRecipe> From<CssBorderCollapse<R>> for CssDeclarationsBlock {
    fn from(css_border_collapse: CssBorderCollapse<R>) -> Self {
        Self::new().push(css_border_collapse)
    }
}
