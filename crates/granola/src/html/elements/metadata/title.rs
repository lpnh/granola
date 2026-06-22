use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<title>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/title)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let title = HtmlTitle::new().id("document_title");
///
/// assert_eq!(title.bake(), r#"<title id="document_title"></title>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let title = HtmlTitle::new().content("On the unabashed art of self-referential examples");
///
/// assert_eq!(
///     title.bake(),
///     r#"<title>On the unabashed art of self-referential examples</title>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <title
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</title>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TitleRecipe, content = Cow<'static, str>)]
pub struct HtmlTitle<R: TitleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlTitle`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let title = title!().id("document_title");
///
/// assert_eq!(title.bake(), r#"<title id="document_title"></title>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let title = title!("On the unabashed art of self-referential examples");
///
/// assert_eq!(
///     title.bake(),
///     r#"<title>On the unabashed art of self-referential examples</title>"#
/// );
/// ```
#[macro_export]
macro_rules! title {
    () => {
        $crate::html::HtmlTitle::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlTitle::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTitle::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlTitle::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlTitle::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTitle::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
