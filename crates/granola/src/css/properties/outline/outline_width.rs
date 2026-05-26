use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `outline-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-width)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline_width: CssOutlineWidth = CssOutlineWidth::new("2px");
///
/// assert_eq!(css_outline_width.bake(), "outline-width: 2px;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline-width: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineWidthTag)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutlineWidth<R: OutlineWidthTag = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: OutlineWidthTag> CssOutlineWidth<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: OutlineWidthTag> From<CssOutlineWidth<R>> for CssDeclaration {
    fn from(css_outline_width: CssOutlineWidth<R>) -> Self {
        Self::new("outline-width", css_outline_width.value)
    }
}

impl<R: OutlineWidthTag> From<CssOutlineWidth<R>> for CssPropertiesList {
    fn from(css_outline_width: CssOutlineWidth<R>) -> Self {
        Self {
            declarations: vec![css_outline_width.into()],
        }
    }
}
