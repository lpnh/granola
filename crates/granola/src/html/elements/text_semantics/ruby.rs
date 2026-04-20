use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait RubyTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
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
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }
}
