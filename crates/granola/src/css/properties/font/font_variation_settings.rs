use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `font-variation-settings` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-variation-settings)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_variation_settings = CssFontVariationSettings::new().content("inherit");
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
/// font-variation-settings: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontVariationSettingsRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontVariationSettings<R: FontVariationSettingsRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: FontVariationSettingsRecipe> From<CssFontVariationSettings<R>> for CssDeclaration {
    fn from(css_font_variation_settings: CssFontVariationSettings<R>) -> Self {
        Self::new(
            "font-variation-settings",
            css_font_variation_settings.bake_recipe().content,
        )
    }
}

impl<R: FontVariationSettingsRecipe> From<CssFontVariationSettings<R>> for CssDeclarationsBlock {
    fn from(css_font_variation_settings: CssFontVariationSettings<R>) -> Self {
        Self::new().push(css_font_variation_settings)
    }
}
