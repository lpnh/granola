use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait WbrTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
}

impl WbrTag for () {}

/// The HTML `<wbr />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/wbr)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let wbr: HtmlWbr = HtmlWbr::new().id("line_break_opportunity");
///
/// assert_eq!(wbr.bake(),
/// r#"<wbr id="line_break_opportunity" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <wbr
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlWbr<M: WbrTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: WbrTag> HtmlWbr<M> {
    pub fn new() -> Self {
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
