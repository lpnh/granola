mod margin_block_end;
pub use margin_block_end::*;
mod margin_bottom;
pub use margin_bottom::*;
mod margin_inline_end;
pub use margin_inline_end::*;
mod margin_left;
pub use margin_left::*;
mod margin_right;
pub use margin_right::*;
mod margin_top;
pub use margin_top::*;

// margin_block
// margin_block_start
// margin_inline
// margin_inline_start

use askama::Template;
use std::marker::PhantomData;

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
/// margin: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginRecipe, content = Bake)]
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
