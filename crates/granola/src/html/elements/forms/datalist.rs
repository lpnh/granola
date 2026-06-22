use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<datalist>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/datalist)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let datalist = HtmlDatalist::new().id("html_data_list");
///
/// assert_eq!(
///     datalist.bake(),
///     r#"<datalist id="html_data_list"></datalist>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let options = [
///     HtmlOption::new().value("Chocolate"),
///     HtmlOption::new().value("Strawberry"),
///     HtmlOption::new().value("Vanilla"),
/// ];
///
/// let datalist = HtmlDatalist::new().content(options).id("ice-cream-flavors");
///
/// assert_eq!(
///     datalist.bake_pretty(),
///     r#"<datalist id="ice-cream-flavors">
///   <option value="Chocolate"></option>
///   <option value="Strawberry"></option>
///   <option value="Vanilla"></option>
/// </datalist>
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <datalist
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</datalist>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DatalistRecipe, content = Options)]
pub struct HtmlDatalist<R: DatalistRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlDatalist`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let datalist = datalist!().id("html_data_list");
///
/// assert_eq!(
///     datalist.bake(),
///     r#"<datalist id="html_data_list"></datalist>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let yes = option!().value("Yes");
/// let no = option!().value("No");
///
/// let datalist = datalist![yes, no].id("binary");
///
/// assert_eq!(datalist.bake(),
/// r#"<datalist id="binary"><option value="Yes"></option><option value="No"></option></datalist>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let options = [
///     option!().value("Chocolate"),
///     option!().value("Strawberry"),
///     option!().value("Vanilla"),
/// ];
///
/// let datalist = datalist!(options).id("ice-cream-flavors");
///
/// assert_eq!(
///     datalist.bake_pretty(),
///     r#"<datalist id="ice-cream-flavors">
///   <option value="Chocolate"></option>
///   <option value="Strawberry"></option>
///   <option value="Vanilla"></option>
/// </datalist>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! datalist {
    () => {
        $crate::html::HtmlDatalist::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDatalist::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDatalist::new().content([$first $(, $rest)*])
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlDatalist::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlDatalist::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDatalist::<$crate::cookbook_type!($($r),+)>::from_cookbook().content([$first $(, $rest)*])
    };
}
