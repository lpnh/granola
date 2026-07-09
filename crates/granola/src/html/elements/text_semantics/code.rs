use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<code>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/code)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let code = HtmlCode::new().id("inline_code");
///
/// assert_eq!(code.bake(), r#"<code id="inline_code"></code>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let code = HtmlCode::new().content("todo!()");
///
/// assert_eq!(code.bake(), r#"<code>todo!()</code>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <code
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</code>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = CodeRecipe, content = Bake)]
pub struct HtmlCode<R: CodeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlCode`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let code = code!().id("inline_code");
///
/// assert_eq!(code.bake(), r#"<code id="inline_code"></code>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let code = code!("todo!()");
///
/// assert_eq!(code.bake(), r#"<code>todo!()</code>"#);
/// ```
#[macro_export]
macro_rules! code {
    () => {
        $crate::html::HtmlCode::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlCode::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlCode::new().content($crate::bake![$first $(, $rest)*])
    };

}
