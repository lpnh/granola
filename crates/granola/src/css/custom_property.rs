use askama::Template;
use std::marker::PhantomData;

use crate::prelude::*;

/// The CSS `--*` custom property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/--*)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_custom_property = CssCustomProperty::new().name("color-background");
///
/// assert_eq!(css_custom_property.bake(), "--color-background");
/// ```
///
/// # Askama template
///
/// ```askama
/// --{{ name }}
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[recipe(name = CustomPropertyRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssCustomProperty<R: CustomPropertyRecipe = ()> {
    _recipe: PhantomData<R>,
    pub name: Bake,
}

impl<R: CustomPropertyRecipe> CssCustomProperty<R> {
    pub fn name(mut self, name: impl Into<Bake>) -> Self {
        self.name = name.into();
        self
    }
}

impl<R: CustomPropertyRecipe> From<CssCustomProperty<R>> for CssFnVar {
    fn from(css_custom_property: CssCustomProperty<R>) -> Self {
        Self::new().custom_property(css_custom_property.name)
    }
}
