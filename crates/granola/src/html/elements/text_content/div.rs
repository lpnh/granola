use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait DivTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlDiv<Self>) -> HtmlDiv<Self> {
        element
    }
}

impl DivTag for () {}

/// The HTML `<div>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/div)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let div: HtmlDiv = HtmlDiv::empty().id("content_division");
///
/// assert_eq!(div.bake(),
/// r#"<div id="content_division"></div>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let save: HtmlButton = HtmlButton::new("Save");
/// let cancel: HtmlButton = HtmlButton::new("Cancel").button_type("button");
///
/// let content = bake_block![save, cancel];
///
/// let div: HtmlDiv = HtmlDiv::new(content).class("flex justify-end gap-2");
///
/// assert_eq!(div.bake(),
/// r#"<div class="flex justify-end gap-2">
///   <button>Save</button>
///   <button type="button">Cancel</button>
/// </div>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <div
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</div>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDiv<M: DivTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DivTag> HtmlDiv<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let element = Self {
            content: content.into(),
            ..Default::default()
        };

        M::recipe(element)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }
}

/// Shorthand for `HtmlDiv<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let div = div!().id("content_division");
///
/// assert_eq!(div.bake(),
/// r#"<div id="content_division"></div>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let save = button!("Save");
/// let cancel = button!("Cancel").button_type("button");
///
/// let div = div!(save, cancel).class("flex justify-end gap-2");
///
/// assert_eq!(div.bake(),
/// r#"<div class="flex justify-end gap-2">
///   <button>Save</button>
///   <button type="button">Cancel</button>
/// </div>"#);
/// ```
#[macro_export]
macro_rules! div {
    () => {
        $crate::html::HtmlDiv::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDiv::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDiv::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDiv::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDiv::<()>::new($crate::bake_inline![$($content),+])
    };
}
