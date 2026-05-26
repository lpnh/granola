use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `outline-style` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-style)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline_style: CssOutlineStyle = CssOutlineStyle::new("0.6em 1.2em");
///
/// assert_eq!(css_outline_style.bake(), "outline-style: 0.6em 1.2em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline-style: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineStyleTag)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutlineStyle<R: OutlineStyleTag = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: OutlineStyleTag> CssOutlineStyle<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: OutlineStyleTag> From<CssOutlineStyle<R>> for CssDeclaration {
    fn from(css_outline_style: CssOutlineStyle<R>) -> Self {
        Self::new("outline-style", css_outline_style.value)
    }
}

impl<R: OutlineStyleTag> From<CssOutlineStyle<R>> for CssPropertiesList {
    fn from(css_outline_style: CssOutlineStyle<R>) -> Self {
        Self {
            declarations: vec![css_outline_style.into()],
        }
    }
}
