use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `align-items` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/align-items)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_align_items: CssAlignItems = CssAlignItems::new("center");
///
/// assert_eq!(css_align_items.bake(), "align-items: center;");
/// ```
///
/// # Askama template
///
/// ```askama
/// align-items: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = AlignItemsRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssAlignItems<R: AlignItemsRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: AlignItemsRecipe> CssAlignItems<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: AlignItemsRecipe> From<CssAlignItems<R>> for CssDeclaration {
    fn from(css_align_items: CssAlignItems<R>) -> Self {
        Self::new("align-items", css_align_items.value)
    }
}

impl<R: AlignItemsRecipe> From<CssAlignItems<R>> for CssPropertiesList {
    fn from(css_align_items: CssAlignItems<R>) -> Self {
        Self {
            declarations: vec![css_align_items.into()],
        }
    }
}
