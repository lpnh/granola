use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait RubyTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlRuby<Self>) -> HtmlRuby<Self> {
        element
    }
}

impl RubyTag for () {}

/// The HTML `<ruby>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ruby)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let ruby: HtmlRuby = HtmlRuby::empty().id("ruby_annotation");
///
/// assert_eq!(ruby.bake(),
/// r#"<ruby id="ruby_annotation"></ruby>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let opening_rp: HtmlRp = HtmlRp::new("(");
/// let rt: HtmlRt = HtmlRt::new("とり");
/// let closing_rp: HtmlRp = HtmlRp::new(")");
///
/// let tori = bake_inline!["鳥", opening_rp, rt, closing_rp];
///
/// let ruby: HtmlRuby = HtmlRuby::new(tori);
///
/// assert_eq!(ruby.bake(),
/// r#"<ruby>鳥<rp>(</rp><rt>とり</rt><rp>)</rp></ruby>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <ruby
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</ruby>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlRuby<M: RubyTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: RubyTag> HtmlRuby<M> {
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

/// Shorthand for `HtmlRuby<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ruby = ruby!().id("ruby_annotation");
///
/// assert_eq!(ruby.bake(),
/// r#"<ruby id="ruby_annotation"></ruby>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let opening_rp = rp!("(");
/// let rt = rt!("とり");
/// let closing_rp = rp!(")");
///
/// let ruby = ruby!(@inline "鳥", opening_rp, rt, closing_rp);
///
/// assert_eq!(ruby.bake(),
/// r#"<ruby>鳥<rp>(</rp><rt>とり</rt><rp>)</rp></ruby>"#);
/// ```
#[macro_export]
macro_rules! ruby {
    () => {
        $crate::html::HtmlRuby::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlRuby::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlRuby::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlRuby::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlRuby::<()>::new($crate::bake_inline![$($content),+])
    };
}
