use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `margin-block-end` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-block-end)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin_block_end = CssMarginBlockEnd::new().content("0");
///
/// assert_eq!(css_margin_block_end.bake(), "margin-block-end: 0;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin-block-end: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginBlockEndRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMarginBlockEnd<R: MarginBlockEndRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: MarginBlockEndRecipe> From<CssMarginBlockEnd<R>> for CssDeclaration {
    fn from(css_margin_block_end: CssMarginBlockEnd<R>) -> Self {
        Self::new(
            "margin-block-end",
            css_margin_block_end.bake_recipe().content,
        )
    }
}

impl<R: MarginBlockEndRecipe> From<CssMarginBlockEnd<R>> for CssDeclarationsBlock {
    fn from(css_margin_block_end: CssMarginBlockEnd<R>) -> Self {
        Self::new().push(css_margin_block_end)
    }
}
