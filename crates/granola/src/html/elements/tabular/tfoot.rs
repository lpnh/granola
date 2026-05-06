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
/// let tfoot: HtmlTfoot = HtmlTfoot::empty().id("table_foot");
///
/// assert_eq!(tfoot.bake(),
/// r#"<tfoot id="table_foot"></tfoot>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let td: HtmlTd = HtmlTd::new("Don't see what you're after? We'll do our best.").colspan(2);
/// let tr: HtmlTr = HtmlTr::new(bake_newline!(td));
///
/// let tfoot: HtmlTfoot = HtmlTfoot::new(tr);
///
/// assert_eq!(tfoot.bake(),
/// r#"<tfoot>
///   <tr>
///     <td colspan="2">Don't see what you're after? We'll do our best.</td>
///   </tr>
/// </tfoot>"#);
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
/// >{{ content | kirei(2) }}</tfoot>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TfootTag, content = TableRows)]
pub struct HtmlTfoot<M: TfootTag = ()> {
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

/// Shorthand for `HtmlTfoot`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tfoot = tfoot!().id("table_foot");
///
/// assert_eq!(tfoot.bake(),
/// r#"<tfoot id="table_foot"></tfoot>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let td = td!("Don't see what you're after? We'll do our best.").colspan(2);
/// let tr = tr!(@newline td);
///
/// let tfoot = tfoot!(tr);
///
/// assert_eq!(tfoot.bake(),
/// r#"<tfoot>
///   <tr>
///     <td colspan="2">Don't see what you're after? We'll do our best.</td>
///   </tr>
/// </tfoot>"#);
/// ```
#[macro_export]
macro_rules! tfoot {
    () => {
        $crate::html::HtmlTfoot::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTfoot::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTfoot::<()>::new([$first $(, $rest)*])
    };
}
