use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `font-feature-settings` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-feature-settings)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_feature_settings = CssFontFeatureSettings::new().content("inherit");
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
/// font-feature-settings: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontFeatureSettingsRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontFeatureSettings<R: FontFeatureSettingsRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: FontFeatureSettingsRecipe> From<CssFontFeatureSettings<R>> for CssDeclaration {
    fn from(css_font_feature_settings: CssFontFeatureSettings<R>) -> Self {
        Self::new(
            "font-feature-settings",
            css_font_feature_settings.bake_recipe().content,
        )
    }
}

impl<R: FontFeatureSettingsRecipe> From<CssFontFeatureSettings<R>> for CssDeclarationsBlock {
    fn from(css_font_feature_settings: CssFontFeatureSettings<R>) -> Self {
        Self::new().push(css_font_feature_settings)
    }
}
