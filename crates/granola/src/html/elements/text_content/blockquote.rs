use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait BlockquoteTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlBlockquote<Self>) -> HtmlBlockquote<Self> {
        element
    }
}

impl BlockquoteTag for () {}

/// The HTML `<blockquote>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let blockquote: HtmlBlockquote = HtmlBlockquote::empty().id("block_quotation");
///
/// assert_eq!(blockquote.bake(),
/// r#"<blockquote id="block_quotation"></blockquote>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let content = bake_block![
///   "The &lt;blockquote&gt; element indicates that the enclosed text is an extended quotation.",
///   "Usually, this is rendered visually by indentation.",
///   "A URL for the source of the quotation may be given using the cite attribute,",
///   "while a text representation of the source can be given using the &lt;cite&gt; element.",
/// ];
///
/// let paragraph: HtmlP = HtmlP::new(content);
///
/// let blockquote: HtmlBlockquote = HtmlBlockquote::new(paragraph)
///     .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote");
///
/// assert_eq!(blockquote.bake(),
/// r#"<blockquote cite="https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote">
///   <p>
///     The &lt;blockquote&gt; element indicates that the enclosed text is an extended quotation.
///     Usually, this is rendered visually by indentation.
///     A URL for the source of the quotation may be given using the cite attribute,
///     while a text representation of the source can be given using the &lt;cite&gt; element.
///   </p>
/// </blockquote>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <blockquote
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</blockquote>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlBlockquote<M: BlockquoteTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: BlockquoteTag> HtmlBlockquote<M> {
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
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote#cite)
    pub fn cite(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("cite", value);
        self
    }
}

/// Shorthand for `HtmlBlockquote<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let blockquote = blockquote!().id("block_quotation");
///
/// assert_eq!(blockquote.bake(),
/// r#"<blockquote id="block_quotation"></blockquote>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let paragraph = p![
///   "The &lt;blockquote&gt; element indicates that the enclosed text is an extended quotation.",
///   "Usually, this is rendered visually by indentation.",
///   "A URL for the source of the quotation may be given using the cite attribute,",
///   "while a text representation of the source can be given using the &lt;cite&gt; element.",
/// ];
///
/// let blockquote = blockquote!(paragraph)
///     .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote");
///
/// assert_eq!(blockquote.bake(),
/// r#"<blockquote cite="https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote">
///   <p>
///     The &lt;blockquote&gt; element indicates that the enclosed text is an extended quotation.
///     Usually, this is rendered visually by indentation.
///     A URL for the source of the quotation may be given using the cite attribute,
///     while a text representation of the source can be given using the &lt;cite&gt; element.
///   </p>
/// </blockquote>"#);
/// ```
#[macro_export]
macro_rules! blockquote {
    () => {
        $crate::html::HtmlBlockquote::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlBlockquote::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlBlockquote::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlBlockquote::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlBlockquote::<()>::new($crate::bake_inline![$($content),+])
    };
}
