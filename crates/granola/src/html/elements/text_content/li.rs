use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait LiTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: menuitem, menuitemcheckbox, menuitemradio, option, none, presentation,
    ///     radio, separator, tab, treeitem
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl LiTag for () {}

/// The HTML `<li>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/li)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let li: HtmlLi = HtmlLi::empty().id("list_item");
///
/// assert_eq!(li.bake(),
/// r#"<li id="list_item"></li>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let sugar: HtmlLi = HtmlLi::new("sugar");
/// let spice: HtmlLi = HtmlLi::new("spice");
///
/// let items = bake_block![sugar, spice];
///
/// assert_eq!(items,
/// r#"<li>sugar</li>
/// <li>spice</li>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <li
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</li>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlLi<M: LiTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: LiTag> HtmlLi<M> {
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

    /// Ordinal value of the list item.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/li#value)
    pub fn value(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("value", value.to_string());
        self
    }
}

/// A collection of HTML `<li>` items.
///
/// The content of [`HtmlMenu`], [`HtmlOl`], or [`HtmlUl`].
///
/// ```askama
/// {%- for li in items %}
/// {{ li -}}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, PartialEq, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ListItems {
    items: Vec<HtmlLi>,
}

impl<I: IntoIterator<Item = HtmlLi>> From<I> for ListItems {
    fn from(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl From<HtmlLi> for ListItems {
    fn from(item: HtmlLi) -> Self {
        Self { items: vec![item] }
    }
}
