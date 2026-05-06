use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

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
/// let code: HtmlCode = HtmlCode::empty().id("inline_code");
///
/// assert_eq!(code.bake(),
/// r#"<code id="inline_code"></code>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let code: HtmlCode = HtmlCode::new("todo!()");
///
/// assert_eq!(code.bake(),
/// r#"<code>todo!()</code>"#);
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
/// >{{ content | kirei(2) }}</code>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = CodeTag, content = Cow<'static, str>)]
pub struct HtmlCode<M: CodeTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
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
/// assert_eq!(code.bake(),
/// r#"<code id="inline_code"></code>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let code = code!("todo!()");
///
/// assert_eq!(code.bake(),
/// r#"<code>todo!()</code>"#);
/// ```
#[macro_export]
macro_rules! code {
    () => {
        $crate::html::HtmlCode::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlCode::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlCode::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlCode::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlCode::<()>::new($crate::bake_inline![$($content),+])
    };
}
