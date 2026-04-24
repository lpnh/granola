use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait NavTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl NavTag for () {}

/// The HTML `<nav>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/nav)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let nav: HtmlNav = HtmlNav::empty().id("navigation_section");
///
/// assert_eq!(nav.bake(),
/// r#"<nav id="navigation_section"></nav>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let location: HtmlA = HtmlA::new("Oak Street, corner of Elm Avenue").href("/location");
/// let menu: HtmlA = HtmlA::new("the menu").href("/menu");
/// let note: HtmlA = HtmlA::new("note").href("/contact");
///
/// let content = bake_block![
///     bake_inline!["You can find us at ", location, "."],
///     bake_inline!["Everything we make and love is on ", menu, "."],
///     bake_inline!["Have a thought? Send us a ", note, "."],
/// ];
///
/// let p: HtmlP = HtmlP::new(content);
/// let nav: HtmlNav = HtmlNav::new(p).aria_label("Site navigation");
///
/// assert_eq!(nav.bake(),
/// r#"<nav aria-label="Site navigation">
///   <p>
///     You can find us at <a href="/location">Oak Street, corner of Elm Avenue</a>.
///     Everything we make and love is on <a href="/menu">the menu</a>.
///     Have a thought? Send us a <a href="/contact">note</a>.
///   </p>
/// </nav>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <nav
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</nav>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlNav<M: NavTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: NavTag> HtmlNav<M> {
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
