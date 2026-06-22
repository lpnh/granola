use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<dl>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dl)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dl = HtmlDl::new().id("description_list");
///
/// assert_eq!(dl.bake(), r#"<dl id="description_list"></dl>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt_1 = HtmlDt::new().content("Hiraeth");
/// let dd_1 =
///     HtmlDd::new().content("A longing for a home that no longer exists, or perhaps never did.");
///
/// let group_1 = bake![dt_1, dd_1];
///
/// let dt_2 = HtmlDt::new().content("Pålegg");
/// let dd_2 = HtmlDd::new().content("Anything and everything you might put on a slice of bread.");
///
/// let group_2 = bake![dt_2, dd_2];
///
/// let list = bake![group_1, "", group_2];
///
/// let dl = HtmlDl::new().content(list);
///
/// assert_eq!(
///     dl.bake_pretty(),
///     r#"<dl>
///   <dt>Hiraeth</dt>
///   <dd>A longing for a home that no longer exists, or perhaps never did.</dd>
///   <dt>Pålegg</dt>
///   <dd>Anything and everything you might put on a slice of bread.</dd>
/// </dl>
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <dl
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</dl>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DlRecipe, content = Cow<'static, str>)]
pub struct HtmlDl<R: DlRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// group, list, none, presentation
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlDl`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dl = dl!().id("description_list");
///
/// assert_eq!(dl.bake(), r#"<dl id="description_list"></dl>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dt_1 = dt!("Hiraeth");
/// let dd_1 = dd!("A longing for a home that no longer exists, or perhaps never did.");
///
/// let group_1 = bake![dt_1, dd_1];
///
/// let dt_2 = dt!("Pålegg");
/// let dd_2 = dd!("Anything and everything you might put on a slice of bread.");
///
/// let group_2 = bake![dt_2, dd_2];
///
/// let dl = dl!(group_1, "", group_2);
///
/// assert_eq!(
///     dl.bake_pretty(),
///     r#"<dl>
///   <dt>Hiraeth</dt>
///   <dd>A longing for a home that no longer exists, or perhaps never did.</dd>
///   <dt>Pålegg</dt>
///   <dd>Anything and everything you might put on a slice of bread.</dd>
/// </dl>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! dl {
    () => {
        $crate::html::HtmlDl::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDl::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDl::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlDl::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlDl::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDl::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
