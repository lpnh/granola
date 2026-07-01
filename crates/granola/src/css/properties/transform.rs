// mod transform_box;
// mod transform_origin;
// mod transform_style;

use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `transform` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/transform)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_transform = CssTransform::new().content("scale(0.97)");
///
/// assert_eq!(css_transform.bake(), "transform: scale(0.97);");
/// ```
///
/// # Askama template
///
/// ```askama
/// transform: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TransformRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTransform<R: TransformRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TransformRecipe> From<CssTransform<R>> for CssDeclaration {
    fn from(css_transform: CssTransform<R>) -> Self {
        Self::new("transform", css_transform.bake_recipe().content)
    }
}

impl<R: TransformRecipe> From<CssTransform<R>> for CssDeclarationsBlock {
    fn from(css_transform: CssTransform<R>) -> Self {
        Self::new().push(css_transform)
    }
}
