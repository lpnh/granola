use askama::Template;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait ColgroupTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = TableColumns;
}

impl ColgroupTag for () {}

/// The HTML `<colgroup>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/colgroup)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let colgroup: HtmlColgroup = HtmlColgroup::empty().id("table_column_group");
///
/// assert_eq!(colgroup.bake(),
/// r#"<colgroup id="table_column_group"></colgroup>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let item: HtmlCol = HtmlCol::new().class("item");
/// let description: HtmlCol = HtmlCol::new().class("description");
///
/// let colgroup: HtmlColgroup = HtmlColgroup::new([item, description]);
///
/// assert_eq!(colgroup.bake(),
/// r#"<colgroup>
///   <col class="item" />
///   <col class="description" />
/// </colgroup>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <colgroup
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</colgroup>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlColgroup<M: ColgroupTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ColgroupTag> HtmlColgroup<M> {
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

    /// Number of columns spanned by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/colgroup#span)
    pub fn span(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("span", value.to_string());
        self
    }
}

/// Shorthand for `HtmlColgroup<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let colgroup = colgroup!().id("table_column_group");
///
/// assert_eq!(colgroup.bake(),
/// r#"<colgroup id="table_column_group"></colgroup>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let item = col!().class("item");
/// let description = col!().class("description");
///
/// let colgroup = colgroup!(item, description);
///
/// assert_eq!(colgroup.bake(),
/// r#"<colgroup>
///   <col class="item" />
///   <col class="description" />
/// </colgroup>"#);
/// ```
#[macro_export]
macro_rules! colgroup {
    () => {
        $crate::html::HtmlColgroup::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlColgroup::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlColgroup::<()>::new([$first $(, $rest)*])
    };
}
