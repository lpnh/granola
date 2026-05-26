use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `justify-content` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/justify-content)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_justify_content: CssJustifyContent = CssJustifyContent::new("center");
///
/// assert_eq!(css_justify_content.bake(), "justify-content: center;");
/// ```
///
/// # Askama template
///
/// ```askama
/// justify-content: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = JustifyContentRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssJustifyContent<R: JustifyContentRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: JustifyContentRecipe> CssJustifyContent<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: JustifyContentRecipe> From<CssJustifyContent<R>> for CssDeclaration {
    fn from(css_justify_content: CssJustifyContent<R>) -> Self {
        Self::new("justify-content", css_justify_content.value)
    }
}

impl<R: JustifyContentRecipe> From<CssJustifyContent<R>> for CssPropertiesList {
    fn from(css_justify_content: CssJustifyContent<R>) -> Self {
        Self {
            declarations: vec![css_justify_content.into()],
        }
    }
}
