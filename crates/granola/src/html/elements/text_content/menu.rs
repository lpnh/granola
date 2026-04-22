use askama::Template;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait MenuTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: directory, group, listbox, menu, menubar, none, presentation,
    ///     radiogroup, tablist, toolbar or tree
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = ListItems;
}

impl MenuTag for () {}

/// The HTML `<menu>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/menu)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let menu: HtmlMenu = HtmlMenu::empty().id("menu");
///
/// assert_eq!(menu.bake(),
/// r#"<menu id="menu"></menu>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let items = [
///   HtmlLi::new("Buy"),
///   HtmlLi::new("Use"),
///   HtmlLi::new("Break"),
///   HtmlLi::new("Fix"),
/// ];
///
/// let technologic: HtmlMenu = HtmlMenu::new(items);
///
/// assert_eq!(technologic.bake(),
/// r#"<menu>
///   <li>Buy</li>
///   <li>Use</li>
///   <li>Break</li>
///   <li>Fix</li>
/// </menu>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <menu
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</menu>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlMenu<M: MenuTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: MenuTag> HtmlMenu<M> {
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
