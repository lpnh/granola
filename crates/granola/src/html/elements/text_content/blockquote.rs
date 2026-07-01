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
/// let blockquote = HtmlBlockquote::new().id("block_quotation");
///
/// assert_eq!(
///     blockquote.bake(),
///     r#"<blockquote id="block_quotation"></blockquote>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let content = bake_ws![
///   "The &lt;blockquote&gt; element indicates that the enclosed text is an extended quotation.",
///   "Usually, this is rendered visually by indentation.",
///   "A URL for the source of the quotation may be given using the cite attribute,",
///   "while a text representation of the source can be given using the &lt;cite&gt; element.",
/// ];
///
/// let paragraph = HtmlP::new().content(content);
///
/// let blockquote = HtmlBlockquote::new().content(paragraph)
///     .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote");
///
/// assert_eq!(
///     blockquote.bake_pretty(),
///     r#"<blockquote cite="https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote">
///   <p>
///     The &lt;blockquote&gt; element indicates that the enclosed text is an
///     extended quotation. Usually, this is rendered visually by indentation. A URL
///     for the source of the quotation may be given using the cite attribute, while
///     a text representation of the source can be given using the &lt;cite&gt;
///     element.
///   </p>
/// </blockquote>
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <blockquote
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</blockquote>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BlockquoteRecipe, content = Bake)]
pub struct HtmlBlockquote<R: BlockquoteRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: BlockquoteAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
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

impl<R: BlockquoteRecipe> HasBlockquoteAttrs for HtmlBlockquote<R> {
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
/// assert_eq!(
///     blockquote.bake(),
///     r#"<blockquote id="block_quotation"></blockquote>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let content = bake_ws![
///   "The &lt;blockquote&gt; element indicates that the enclosed text is an extended quotation.",
///   "Usually, this is rendered visually by indentation.",
///   "A URL for the source of the quotation may be given using the cite attribute,",
///   "while a text representation of the source can be given using the &lt;cite&gt; element.",
/// ];
///
/// let paragraph = p!(content);
///
/// let blockquote = blockquote!(paragraph)
///     .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote");
///
/// assert_eq!(
///     blockquote.bake_pretty(),
///     r#"<blockquote cite="https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote">
///   <p>
///     The &lt;blockquote&gt; element indicates that the enclosed text is an
///     extended quotation. Usually, this is rendered visually by indentation. A URL
///     for the source of the quotation may be given using the cite attribute, while
///     a text representation of the source can be given using the &lt;cite&gt;
///     element.
///   </p>
/// </blockquote>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! blockquote {
    () => {
        $crate::html::HtmlBlockquote::new()
    };

    ($content:expr $(,)?) => {
        $crate::html::HtmlBlockquote::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlBlockquote::new().content($crate::bake![$first $(, $rest)*])
    };
}
