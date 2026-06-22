use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `scroll-margin-block` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/scroll-margin-block)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_scroll_margin_block = CssScrollMarginBlock::new().content("5ex");
///
/// assert_eq!(css_scroll_margin_block.bake(), "scroll-margin-block: 5ex;");
/// ```
///
/// # Askama template
///
/// ```askama
/// scroll-margin-block: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ScrollMarginBlockRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssScrollMarginBlock<R: ScrollMarginBlockRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: ScrollMarginBlockRecipe> From<CssScrollMarginBlock<R>> for CssDeclaration {
    fn from(css_scroll_margin_block: CssScrollMarginBlock<R>) -> Self {
        Self::new(
            "scroll-margin-block",
            css_scroll_margin_block.bake_recipe().content,
        )
    }
}

impl<R: ScrollMarginBlockRecipe> From<CssScrollMarginBlock<R>> for CssDeclarationsBlock {
    fn from(css_scroll_margin_block: CssScrollMarginBlock<R>) -> Self {
        Self::new().push(css_scroll_margin_block)
    }
}
