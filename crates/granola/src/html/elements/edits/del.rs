use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait DelTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlDel<Self>) -> HtmlDel<Self> {
        element
    }
}

impl DelTag for () {}

/// The HTML `<del>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let del: HtmlDel = HtmlDel::empty().id("deleted_text");
///
/// assert_eq!(del.bake(),
/// r#"<del id="deleted_text"></del>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let del: HtmlDel = HtmlDel::new(bake_newline!("try!"))
///     .datetime("2019-11-07")
///     .cite("https://github.com/rust-lang/rust/pull/62672/");
///
/// assert_eq!(del.bake(),
/// r#"<del datetime="2019-11-07" cite="https://github.com/rust-lang/rust/pull/62672/">
///   try!
/// </del>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <del
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</del>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDel<M: DelTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DelTag> HtmlDel<M> {
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
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del#cite)
    pub fn cite(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("cite", value);
        self
    }

    /// Date and (optionally) time of the change.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del#datetime)
    pub fn datetime(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("datetime", value);
        self
    }
}

/// Shorthand for `HtmlDel<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let del = del!().id("deleted_text");
///
/// assert_eq!(del.bake(),
/// r#"<del id="deleted_text"></del>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let del = del!(@newline "try!")
///     .datetime("2019-11-07")
///     .cite("https://github.com/rust-lang/rust/pull/62672/");
///
/// assert_eq!(del.bake(),
/// r#"<del datetime="2019-11-07" cite="https://github.com/rust-lang/rust/pull/62672/">
///   try!
/// </del>"#);
/// ```
#[macro_export]
macro_rules! del {
    () => {
        $crate::html::HtmlDel::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDel::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDel::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDel::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDel::<()>::new($crate::bake_inline![$($content),+])
    };
}
