use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `background-color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/background-color)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_background_color: CssBackgroundColor = CssBackgroundColor::new("transparent");
///
/// assert_eq!(
///     css_background_color.bake(),
///     "background-color: transparent;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// background-color: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BackgroundColorTag)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBackgroundColor<R: BackgroundColorTag = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: BackgroundColorTag> CssBackgroundColor<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: BackgroundColorTag> From<CssBackgroundColor<R>> for CssDeclaration {
    fn from(css_background_color: CssBackgroundColor<R>) -> Self {
        Self::new("background-color", css_background_color.value)
    }
}

impl<R: BackgroundColorTag> From<CssBackgroundColor<R>> for CssPropertiesList {
    fn from(css_background_color: CssBackgroundColor<R>) -> Self {
        Self {
            declarations: vec![css_background_color.into()],
        }
    }
}
