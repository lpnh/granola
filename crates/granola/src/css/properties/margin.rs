mod margin_block_end;
pub use margin_block_end::*;
mod margin_inline_end;
pub use margin_inline_end::*;
mod margin_top;
pub use margin_top::*;

// margin_block
// margin_block_start
// margin_bottom
// margin_inline
// margin_inline_start
// margin_left
// margin_right

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `margin` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin = CssMargin::new().content("0");
///
/// assert_eq!(css_margin.bake(), "margin: 0;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMargin<R: MarginRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: MarginRecipe> From<CssMargin<R>> for CssDeclaration {
    fn from(css_margin: CssMargin<R>) -> Self {
        Self::new("margin", css_margin.bake_recipe().content)
    }
}

impl<R: MarginRecipe> From<CssMargin<R>> for CssDeclarationsBlock {
    fn from(css_margin: CssMargin<R>) -> Self {
        Self::new().push(css_margin)
    }
}
