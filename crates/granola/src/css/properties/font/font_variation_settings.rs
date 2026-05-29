use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `font-variation-settings` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-variation-settings)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_variation_settings: CssFontVariationSettings =
///     CssFontVariationSettings::new("inherit");
///
/// assert_eq!(
///     css_font_variation_settings.bake(),
///     "font-variation-settings: inherit;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// font-variation-settings: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontVariationSettingsRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontVariationSettings<R: FontVariationSettingsRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: FontVariationSettingsRecipe> CssFontVariationSettings<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: FontVariationSettingsRecipe> From<CssFontVariationSettings<R>> for CssDeclaration {
    fn from(css_font_variation_settings: CssFontVariationSettings<R>) -> Self {
        Self::new("font-variation-settings", css_font_variation_settings.value)
    }
}

impl<R, B> From<CssFontVariationSettings<R>> for CssDeclarationsBlock<B>
where
    R: FontVariationSettingsRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_font_variation_settings: CssFontVariationSettings<R>) -> Self {
        Self::new().push(css_font_variation_settings)
    }
}
