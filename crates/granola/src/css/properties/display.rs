use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The `display` CSS property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/display)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_display: CssDisplay = CssDisplay::new("inline flex");
///
/// assert_eq!(css_display.bake(), "display: inline flex;");
/// ```
///
/// # Askama template
///
/// ```askama
/// display: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = DisplayTag)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssDisplay<R: DisplayTag = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: DisplayTag> CssDisplay<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: DisplayTag> From<CssDisplay<R>> for CssDeclaration {
    fn from(css_display: CssDisplay<R>) -> Self {
        Self::new("display", css_display.value)
    }
}

impl<R: DisplayTag> From<CssDisplay<R>> for CssPropertiesList {
    fn from(css_display: CssDisplay<R>) -> Self {
        Self {
            declarations: vec![css_display.into()],
        }
    }
}
