use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- attrs -}}
///   {{- specific_attrs -}}
/// >{{ content | kirei(2) }}</blockquote>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BlockquoteTag, content = Cow<'static, str>, specific = BlockquoteAttrs)]
pub struct HtmlBlockquote<M: BlockquoteTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
    pub specific_attrs: BlockquoteAttrs,
}

/// The HTML `<blockquote>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- cite | bake_attr("cite") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct BlockquoteAttrs {
    pub cite: Option<Cow<'static, str>>,
}

pub trait HasBlockquoteAttrs: Sized {
    fn blockquote_attrs_mut(&mut self) -> &mut BlockquoteAttrs;

    /// Link to the source of the quotation or more information about the edit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote#cite)
    fn cite(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.blockquote_attrs_mut().cite = Some(value.into());
        self
    }
}

impl HasBlockquoteAttrs for BlockquoteAttrs {
    fn blockquote_attrs_mut(&mut self) -> &mut BlockquoteAttrs {
        self
    }
}

impl HasBlockquoteAttrs for &mut BlockquoteAttrs {
    fn blockquote_attrs_mut(&mut self) -> &mut BlockquoteAttrs {
        self
    }
}

impl<M: BlockquoteTag> HasBlockquoteAttrs for HtmlBlockquote<M> {
    fn blockquote_attrs_mut(&mut self) -> &mut BlockquoteAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlBlockquote`.
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
