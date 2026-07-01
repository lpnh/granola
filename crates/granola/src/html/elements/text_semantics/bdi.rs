use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<bdi>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdi)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdi = HtmlBdi::new().id("bidirectional_isolate");
///
/// assert_eq!(bdi.bake(), r#"<bdi id="bidirectional_isolate"></bdi>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let gal = HtmlBdi::new().content("גל גדות");
///
/// let notification = bake![gal, " liked your post"];
///
/// assert_eq!(notification, r#"<bdi>גל גדות</bdi> liked your post"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <bdi
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</bdi>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BdiRecipe, content = Bake)]
pub struct HtmlBdi<R: BdiRecipe = ()> {
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

/// Shorthand for `HtmlBdi`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let bdi = bdi!().id("bidirectional_isolate");
///
/// assert_eq!(bdi.bake(), r#"<bdi id="bidirectional_isolate"></bdi>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let gal = bdi!("גל גדות");
///
/// let notification = bake![gal, " liked your post"];
///
/// assert_eq!(notification, r#"<bdi>גל גדות</bdi> liked your post"#);
/// ```
#[macro_export]
macro_rules! bdi {
    () => {
        $crate::html::HtmlBdi::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlBdi::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlBdi::new().content($crate::bake![$first $(, $rest)*])
    };

}
