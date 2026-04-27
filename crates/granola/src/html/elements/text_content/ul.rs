use askama::Template;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait UlTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: directory, group, listbox, menu, menubar, none, presentation,
    ///     radiogroup, tablist, toolbar, tree
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = ListItems;
}

impl UlTag for () {}

/// The HTML `<ul>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ul)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let ul: HtmlUl = HtmlUl::empty().id("unordered_list");
///
/// assert_eq!(ul.bake(),
/// r#"<ul id="unordered_list"></ul>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let items = [
///   HtmlLi::new("sugar"),
///   HtmlLi::new("spice"),
///   HtmlLi::new("everything nice"),
/// ];
///
/// let ingredients: HtmlUl = HtmlUl::new(items);
///
/// assert_eq!(ingredients.bake(),
/// r#"<ul>
///   <li>sugar</li>
///   <li>spice</li>
///   <li>everything nice</li>
/// </ul>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <ul
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</ul>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlUl<M: UlTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: UlTag> HtmlUl<M> {
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

/// Shorthand for `HtmlUl<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ul = ul!().id("unordered_list");
///
/// assert_eq!(ul.bake(),
/// r#"<ul id="unordered_list"></ul>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let items = [
///   li!("sugar"),
///   li!("spice"),
///   li!("everything nice"),
/// ];
///
/// let ingredients = ul!(items);
///
/// assert_eq!(ingredients.bake(),
/// r#"<ul>
///   <li>sugar</li>
///   <li>spice</li>
///   <li>everything nice</li>
/// </ul>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sugar = li!("sugar");
/// let spice = li!("spice");
///
/// let ingredients = ul!(sugar, spice);
///
/// assert_eq!(ingredients.bake(),
/// r#"<ul>
///   <li>sugar</li>
///   <li>spice</li>
/// </ul>"#);
/// ```
#[macro_export]
macro_rules! ul {
    () => {
        $crate::html::HtmlUl::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlUl::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlUl::<()>::new([$first $(, $rest)*])
    };
}
