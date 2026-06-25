use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS custom (`--*`) property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/--*)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_custom_property = CssCustomProperty::new()
///     .name("color-background")
///     .content("initial");
///
/// assert_eq!(css_custom_property.bake(), "--color-background: initial;");
/// ```
///
/// # Askama template
///
/// ```askama
/// --{{ name }}: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = CustomPropertyRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssCustomProperty<R: CustomPropertyRecipe = ()> {
    _recipe: PhantomData<R>,
    pub name: Cow<'static, str>,
    pub content: R::Content,
}

impl<R: CustomPropertyRecipe> CssCustomProperty<R> {
    pub fn name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name = name.into();
        self
    }
}

impl<R: CustomPropertyRecipe> From<CssCustomProperty<R>> for CssDeclaration {
    fn from(css_custom_property: CssCustomProperty<R>) -> Self {
        let css_custom_property = css_custom_property.bake_recipe();
        Self::new(
            format!("--{}", css_custom_property.name),
            css_custom_property.content,
        )
    }
}

impl<R: CustomPropertyRecipe> From<CssCustomProperty<R>> for CssDeclarationsBlock {
    fn from(css_custom_property: CssCustomProperty<R>) -> Self {
        Self::new().push(css_custom_property)
    }
}
