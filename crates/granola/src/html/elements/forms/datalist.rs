use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait DatalistTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Options;
}

impl DatalistTag for () {}

/// The HTML `<datalist>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/datalist)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let datalist: HtmlDatalist = HtmlDatalist::empty().id("html_data_list");
///
/// assert_eq!(datalist.bake(),
/// r#"<datalist id="html_data_list"></datalist>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let options = [
///     HtmlOption::from_value("Chocolate"),
///     HtmlOption::from_value("Strawberry"),
///     HtmlOption::from_value("Vanilla"),
/// ];
///
/// let datalist: HtmlDatalist = HtmlDatalist::new(options).id("ice-cream-flavors");
///
/// assert_eq!(datalist.bake(),
/// r#"<datalist id="ice-cream-flavors">
///   <option value="Chocolate"></option>
///   <option value="Strawberry"></option>
///   <option value="Vanilla"></option>
/// </datalist>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <datalist
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</datalist>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDatalist<M: DatalistTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DatalistTag> HtmlDatalist<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }
}

/// Shorthand for `HtmlDatalist<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let datalist = datalist!().id("html_data_list");
///
/// assert_eq!(datalist.bake(),
/// r#"<datalist id="html_data_list"></datalist>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let yes = option!(@from_value "Yes");
/// let no = option!(@from_value "No");
///
/// let datalist = datalist![yes, no].id("binary");
///
/// assert_eq!(datalist.bake(),
/// r#"<datalist id="binary">
///   <option value="Yes"></option>
///   <option value="No"></option>
/// </datalist>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let options = [
///     option!(@from_value "Chocolate"),
///     option!(@from_value "Strawberry"),
///     option!(@from_value "Vanilla"),
/// ];
///
/// let datalist = datalist!(options).id("ice-cream-flavors");
///
/// assert_eq!(datalist.bake(),
/// r#"<datalist id="ice-cream-flavors">
///   <option value="Chocolate"></option>
///   <option value="Strawberry"></option>
///   <option value="Vanilla"></option>
/// </datalist>"#);
/// ```
#[macro_export]
macro_rules! datalist {
    () => {
        $crate::html::HtmlDatalist::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDatalist::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDatalist::<()>::new([$first $(, $rest)*])
    };
}
