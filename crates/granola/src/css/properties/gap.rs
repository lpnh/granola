use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `gap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/gap)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_gap = CssGap::new().content("1rem");
///
/// assert_eq!(css_gap.bake(), "gap: 1rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// gap: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = GapRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssGap<R: GapRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: GapRecipe> From<CssGap<R>> for CssDeclaration {
    fn from(css_gap: CssGap<R>) -> Self {
        Self::new("gap", css_gap.bake_recipe().content)
    }
}

impl<R: GapRecipe> From<CssGap<R>> for CssDeclarationsBlock {
    fn from(css_gap: CssGap<R>) -> Self {
        Self::new().push(css_gap)
    }
}
