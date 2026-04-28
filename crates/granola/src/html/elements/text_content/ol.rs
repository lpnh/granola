use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait OlTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: directory, group, listbox, menu, menubar, none, presentation,
    ///     radiogroup, tablist, toolbar, tree
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = ListItems;
}

impl OlTag for () {}

/// The HTML `<ol>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let ol: HtmlOl = HtmlOl::empty().id("ordered_list");
///
/// assert_eq!(ol.bake(),
/// r#"<ol id="ordered_list"></ol>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let items = [
///   HtmlLi::new("Add the sugar"),
///   HtmlLi::new("Coat with spice"),
///   HtmlLi::new("Fold in everything nice"),
/// ];
///
/// let instructions: HtmlOl = HtmlOl::new(items);
///
/// assert_eq!(instructions.bake(),
/// r#"<ol>
///   <li>Add the sugar</li>
///   <li>Coat with spice</li>
///   <li>Fold in everything nice</li>
/// </ol>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <ol
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</ol>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlOl<M: OlTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: OlTag> HtmlOl<M> {
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

    /// Number the list backwards.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol#reversed)
    pub fn reversed(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("reversed");
        }
        self
    }

    /// Starting value of the list.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol#start)
    pub fn start(mut self, value: i32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("start", value.to_string());
        self
    }

    /// Kind of list marker.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol#type)
    pub fn list_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("type", value);
        self
    }
}

/// Shorthand for `HtmlOl<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ol = ol!().id("ordered_list");
///
/// assert_eq!(ol.bake(),
/// r#"<ol id="ordered_list"></ol>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let items = [
///   li!("Add the sugar"),
///   li!("Coat with spice"),
///   li!("Fold in everything nice"),
/// ];
///
/// let instructions = ol!(items);
///
/// assert_eq!(instructions.bake(),
/// r#"<ol>
///   <li>Add the sugar</li>
///   <li>Coat with spice</li>
///   <li>Fold in everything nice</li>
/// </ol>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sugar = li!("Add the sugar");
/// let spice = li!("Coat with spice");
///
/// let instructions = ol!(sugar, spice);
///
/// assert_eq!(instructions.bake(),
/// r#"<ol>
///   <li>Add the sugar</li>
///   <li>Coat with spice</li>
/// </ol>"#);
/// ```
#[macro_export]
macro_rules! ol {
    () => {
        $crate::html::HtmlOl::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlOl::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlOl::<()>::new([$first $(, $rest)*])
    };
}
