use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `align-items` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/align-items)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_align_items = CssAlignItems::new().content("center");
///
/// assert_eq!(css_align_items.bake(), "align-items: center;");
/// ```
///
/// # Askama template
///
/// ```askama
/// align-items: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = AlignItemsRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssAlignItems<R: AlignItemsRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: AlignItemsRecipe> From<CssAlignItems<R>> for CssDeclaration {
    fn from(css_align_items: CssAlignItems<R>) -> Self {
        Self::new("align-items", css_align_items.bake_recipe().content)
    }
}

impl<R: AlignItemsRecipe> From<CssAlignItems<R>> for CssDeclarationsBlock {
    fn from(css_align_items: CssAlignItems<R>) -> Self {
        Self::new().push(css_align_items)
    }
}
