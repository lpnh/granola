use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `text-indent` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-indent)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_indent = CssTextIndent::new().content("0");
///
/// assert_eq!(css_text_indent.bake(), "text-indent: 0;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-indent: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextIndentRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextIndent<R: TextIndentRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TextIndentRecipe> From<CssTextIndent<R>> for CssDeclaration {
    fn from(css_text_indent: CssTextIndent<R>) -> Self {
        Self::new("text-indent", css_text_indent.bake_recipe().content)
    }
}

impl<R: TextIndentRecipe> From<CssTextIndent<R>> for CssDeclarationsBlock {
    fn from(css_text_indent: CssTextIndent<R>) -> Self {
        Self::new().push(css_text_indent)
    }
}
