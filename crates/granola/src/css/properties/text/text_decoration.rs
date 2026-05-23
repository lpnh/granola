use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `text-decoration` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-decoration)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_decoration: CssTextDecoration = CssTextDecoration::new("none");
///
/// assert_eq!(css_text_decoration.bake(),
/// "text-decoration: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-decoration: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextDecorationTag)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextDecoration<R: TextDecorationTag = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TextDecorationTag> CssTextDecoration<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TextDecorationTag> From<CssTextDecoration<R>> for CssDeclaration {
    fn from(css_text_decoration: CssTextDecoration<R>) -> Self {
        Self::new("text-decoration", css_text_decoration.value)
    }
}

impl<R: TextDecorationTag> From<CssTextDecoration<R>> for CssPropertiesList {
    fn from(css_text_decoration: CssTextDecoration<R>) -> Self {
        Self {
            declarations: vec![css_text_decoration.into()],
        }
    }
}
