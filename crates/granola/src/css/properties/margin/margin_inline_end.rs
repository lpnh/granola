use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `margin-inline-end` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-inline-end)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin_inline_end = CssMarginInlineEnd::new().content("4px");
///
/// assert_eq!(css_margin_inline_end.bake(), "margin-inline-end: 4px;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin-inline-end: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginInlineEndRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMarginInlineEnd<R: MarginInlineEndRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: MarginInlineEndRecipe> From<CssMarginInlineEnd<R>> for CssDeclaration {
    fn from(css_margin_inline_end: CssMarginInlineEnd<R>) -> Self {
        Self::new(
            "margin-inline-end",
            css_margin_inline_end.bake_recipe().content,
        )
    }
}

impl<R: MarginInlineEndRecipe> From<CssMarginInlineEnd<R>> for CssDeclarationsBlock {
    fn from(css_margin_inline_end: CssMarginInlineEnd<R>) -> Self {
        Self::new().push(css_margin_inline_end)
    }
}
