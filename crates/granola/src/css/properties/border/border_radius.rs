use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `border-radius` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-radius)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_border_radius: CssBorderRadius = CssBorderRadius::new("0.4em");
///
/// assert_eq!(css_border_radius.bake(),
/// "border-radius: 0.4em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// border-radius: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BorderRadiusTag)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBorderRadius<R: BorderRadiusTag = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: BorderRadiusTag> CssBorderRadius<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: BorderRadiusTag> From<CssBorderRadius<R>> for CssDeclaration {
    fn from(css_border_radius: CssBorderRadius<R>) -> Self {
        Self::new("border-radius", css_border_radius.value)
    }
}

impl<R: BorderRadiusTag> From<CssBorderRadius<R>> for CssPropertiesList {
    fn from(css_border_radius: CssBorderRadius<R>) -> Self {
        Self {
            declarations: vec![css_border_radius.into()],
        }
    }
}
