use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `white-space` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/white-space)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_white_space: CssWhiteSpace = CssWhiteSpace::new("nowrap");
///
/// assert_eq!(css_white_space.bake(), "white-space: nowrap;");
/// ```
///
/// # Askama template
///
/// ```askama
/// white-space: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = WhiteSpaceRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssWhiteSpace<R: WhiteSpaceRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: WhiteSpaceRecipe> CssWhiteSpace<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: WhiteSpaceRecipe> From<CssWhiteSpace<R>> for CssDeclaration {
    fn from(css_white_space: CssWhiteSpace<R>) -> Self {
        Self::new("white-space", css_white_space.value)
    }
}

impl<R: WhiteSpaceRecipe> From<CssWhiteSpace<R>> for CssPropertiesList {
    fn from(css_white_space: CssWhiteSpace<R>) -> Self {
        Self {
            declarations: vec![css_white_space.into()],
        }
    }
}
