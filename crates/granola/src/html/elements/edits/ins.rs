use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait InsTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlIns<Self>) -> HtmlIns<Self> {
        element
    }
}

impl InsTag for () {}

/// The HTML `<ins>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let ins: HtmlIns = HtmlIns::empty().id("inserted_text");
///
/// assert_eq!(ins.bake(),
/// r#"<ins id="inserted_text"></ins>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let ins: HtmlIns = HtmlIns::new(bake_newline!("?"))
///     .datetime("2016-11-10")
///     .cite("https://blog.rust-lang.org/2016/11/10/Rust-1.13/");
///
/// assert_eq!(ins.bake(),
/// r#"<ins datetime="2016-11-10" cite="https://blog.rust-lang.org/2016/11/10/Rust-1.13/">
///   ?
/// </ins>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <ins
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</ins>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlIns<M: InsTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: InsTag> HtmlIns<M> {
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

    /// Link to the source of the quotation or more information about the edit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins#cite)
    pub fn cite(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("cite", value);
        self
    }

    /// Date and (optionally) time of the change.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins#datetime)
    pub fn datetime(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("datetime", value);
        self
    }
}

/// Shorthand for `HtmlIns<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ins = ins!().id("inserted_text");
///
/// assert_eq!(ins.bake(),
/// r#"<ins id="inserted_text"></ins>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ins = ins!(@newline "?")
///     .datetime("2016-11-10")
///     .cite("https://blog.rust-lang.org/2016/11/10/Rust-1.13/");
///
/// assert_eq!(ins.bake(),
/// r#"<ins datetime="2016-11-10" cite="https://blog.rust-lang.org/2016/11/10/Rust-1.13/">
///   ?
/// </ins>"#);
/// ```
#[macro_export]
macro_rules! ins {
    () => {
        $crate::html::HtmlIns::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlIns::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlIns::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlIns::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlIns::<()>::new($crate::bake_inline![$($content),+])
    };
}
