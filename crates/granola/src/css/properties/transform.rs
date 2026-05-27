// mod transform_box;
// mod transform_origin;
// mod transform_style;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `transform` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/transform)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_transform: CssTransform = CssTransform::new("scale(0.97)");
///
/// assert_eq!(css_transform.bake(), "transform: scale(0.97);");
/// ```
///
/// # Askama template
///
/// ```askama
/// transform: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TransformRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTransform<R: TransformRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TransformRecipe> CssTransform<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TransformRecipe> From<CssTransform<R>> for CssDeclaration {
    fn from(css_transform: CssTransform<R>) -> Self {
        Self::new("transform", css_transform.value)
    }
}

impl<R: TransformRecipe> From<CssTransform<R>> for CssDeclarationsBlock {
    fn from(css_transform: CssTransform<R>) -> Self {
        Self {
            declarations: vec![css_transform.into()],
        }
    }
}
