use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<h3>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let h3 = HtmlH3::new().id("html_section_heading");
///
/// assert_eq!(h3.bake(), r#"<h3 id="html_section_heading"></h3>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let panic = HtmlCode::new().content("panic!");
///
/// let content = bake_inline!["Unrecoverable Errors with ", panic];
///
/// let h3 = HtmlH3::new().content(content);
///
/// assert_eq!(
///     h3.bake(),
///     r#"<h3>Unrecoverable Errors with <code>panic!</code></h3>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <h3
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</h3>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = H3Recipe, content = Cow<'static, str>)]
pub struct HtmlH3<R: H3Recipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// tab, presentation or none
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlH3`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h3 = h3!().id("html_section_heading");
///
/// assert_eq!(h3.bake(), r#"<h3 id="html_section_heading"></h3>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let panic = code!("panic!");
///
/// let h3 = h3!(@inline "Unrecoverable Errors with ", panic);
///
/// assert_eq!(h3.bake(),
/// r#"<h3>Unrecoverable Errors with <code>panic!</code></h3>"#);
/// ```
#[macro_export]
macro_rules! h3 {
    () => {
        $crate::html::HtmlH3::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlH3::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlH3::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content:expr $(,)?) => {
        $crate::html::HtmlH3::new().content($crate::bake_newline!($content))
    };
    (@inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlH3::new().content($crate::bake_inline![$($content),+])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlH3::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlH3::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlH3::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlH3::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlH3::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_inline![$($content),+])
    };
}
