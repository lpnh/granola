use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `font-feature-settings` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-feature-settings)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_feature_settings: CssFontFeatureSettings = CssFontFeatureSettings::new("inherit");
///
/// assert_eq!(
///     css_font_feature_settings.bake(),
///     "font-feature-settings: inherit;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// font-feature-settings: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontFeatureSettingsRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontFeatureSettings<R: FontFeatureSettingsRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: FontFeatureSettingsRecipe> CssFontFeatureSettings<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: FontFeatureSettingsRecipe> From<CssFontFeatureSettings<R>> for CssDeclaration {
    fn from(css_font_feature_settings: CssFontFeatureSettings<R>) -> Self {
        Self::new("font-feature-settings", css_font_feature_settings.value)
    }
}

impl<R, B> From<CssFontFeatureSettings<R>> for CssDeclarationsBlock<B>
where
    R: FontFeatureSettingsRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_font_feature_settings: CssFontFeatureSettings<R>) -> Self {
        Self::new().push(css_font_feature_settings)
    }
}
