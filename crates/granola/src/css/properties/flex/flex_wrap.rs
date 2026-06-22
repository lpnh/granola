use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `flex-wrap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/flex-wrap)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_flex_wrap = CssFlexWrap::new().content("nowrap");
///
/// assert_eq!(css_flex_wrap.bake(), "flex-wrap: nowrap;");
/// ```
///
/// # Askama template
///
/// ```askama
/// flex-wrap: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FlexWrapRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFlexWrap<R: FlexWrapRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: FlexWrapRecipe> From<CssFlexWrap<R>> for CssDeclaration {
    fn from(css_flex_wrap: CssFlexWrap<R>) -> Self {
        Self::new("flex-wrap", css_flex_wrap.bake_recipe().content)
    }
}

impl<R: FlexWrapRecipe> From<CssFlexWrap<R>> for CssDeclarationsBlock {
    fn from(css_flex_wrap: CssFlexWrap<R>) -> Self {
        Self::new().push(css_flex_wrap)
    }
}
