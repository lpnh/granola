use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<tfoot>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tfoot)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let tfoot = HtmlTfoot::new().id("table_foot");
///
/// assert_eq!(tfoot.bake(), r#"<tfoot id="table_foot"></tfoot>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let td = HtmlTd::new()
///     .content("Don't see what you're after? We'll do our best.")
///     .colspan(2);
/// let tr = HtmlTr::new().content(td);
///
/// let tfoot = HtmlTfoot::new().content(tr);
///
/// assert_eq!(
///     tfoot.bake(),
///     r#"<tfoot><tr><td colspan="2">Don't see what you're after? We'll do our best.</td></tr></tfoot>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <tfoot
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</tfoot>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TfootRecipe, content = TableRows)]
pub struct HtmlTfoot<R: TfootRecipe = ()> {
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

/// Shorthand for `HtmlTfoot`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tfoot = tfoot!().id("table_foot");
///
/// assert_eq!(tfoot.bake(), r#"<tfoot id="table_foot"></tfoot>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let td = td!("Don't see what you're after? We'll do our best.").colspan(2);
/// let tr = tr!(td);
///
/// let tfoot = tfoot!(tr);
///
/// assert_eq!(
///     tfoot.bake(),
///     r#"<tfoot><tr><td colspan="2">Don't see what you're after? We'll do our best.</td></tr></tfoot>"#
/// );
/// ```
#[macro_export]
macro_rules! tfoot {
    () => {
        $crate::html::HtmlTfoot::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlTfoot::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTfoot::new().content([$first $(, $rest)*])
    };
    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlTfoot::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; $content:expr $(,)?) => {
        $crate::html::HtmlTfoot::<$r>::from_cookbook().content($content)
    };
    (@cookbook $r:ty ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTfoot::<$r>::from_cookbook().content([$first $(, $rest)*])
    };
}
