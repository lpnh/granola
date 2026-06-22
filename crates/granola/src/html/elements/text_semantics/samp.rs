use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<samp>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/samp)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let samp = HtmlSamp::new().id("sample_output");
///
/// assert_eq!(samp.bake(), r#"<samp id="sample_output"></samp>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let error = HtmlSamp::new().content("No such file or directory");
///
/// assert_eq!(error.bake(), r#"<samp>No such file or directory</samp>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <samp
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</samp>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SampRecipe, content = Cow<'static, str>)]
pub struct HtmlSamp<R: SampRecipe = ()> {
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

/// Shorthand for `HtmlSamp`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let samp = samp!().id("sample_output");
///
/// assert_eq!(samp.bake(), r#"<samp id="sample_output"></samp>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let error = samp!("No such file or directory");
///
/// assert_eq!(error.bake(), r#"<samp>No such file or directory</samp>"#);
/// ```
#[macro_export]
macro_rules! samp {
    () => {
        $crate::html::HtmlSamp::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlSamp::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlSamp::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlSamp::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlSamp::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlSamp::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
