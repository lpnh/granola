use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<h4>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let h4: HtmlH4 = HtmlH4::empty().id("html_section_heading");
///
/// assert_eq!(h4.bake(), r#"<h4 id="html_section_heading"></h4>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let h4: HtmlH4 = HtmlH4::new("In fable and literature");
///
/// assert_eq!(h4.bake(), r#"<h4>In fable and literature</h4>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <h4
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</h4>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = H4Recipe, content = Cow<'static, str>)]
pub struct HtmlH4<R: H4Recipe = ()> {
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

/// Shorthand for `HtmlH4`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h4 = h4!().id("html_section_heading");
///
/// assert_eq!(h4.bake(), r#"<h4 id="html_section_heading"></h4>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h4 = h4!("In fable and literature");
///
/// assert_eq!(h4.bake(), r#"<h4>In fable and literature</h4>"#);
/// ```
#[macro_export]
macro_rules! h4 {
    () => {
        $crate::html::HtmlH4::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlH4::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlH4::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlH4::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlH4::<()>::new($crate::bake_inline![$($content),+])
    };
}
