use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait ColTag: Default + Clone + Debug + 'static {
    fn recipe(element: HtmlCol<Self>) -> HtmlCol<Self> {
        element
    }
}

impl ColTag for () {}

/// The HTML `<col />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/col)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let col: HtmlCol = HtmlCol::new().id("table_column");
///
/// assert_eq!(col.bake(),
/// r#"<col id="table_column" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let item: HtmlCol = HtmlCol::new().class("item");
/// let description: HtmlCol = HtmlCol::new().class("description");
///
/// let cols = bake_block![item, description];
///
/// assert_eq!(cols,
/// r#"<col class="item" />
/// <col class="description" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <col
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlCol<M: ColTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ColTag> HtmlCol<M> {
    pub fn new() -> Self {
        let element = Self::default();

        M::recipe(element)
    }

    /// Number of columns spanned by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/col#span)
    pub fn span(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("span", value.to_string());
        self
    }
}

/// A collection of HTML `<col>` items.
///
/// The content of [`HtmlColgroup`].
///
/// ```askama
/// {%- for col in items %}
/// {{ col -}}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, PartialEq, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TableColumns {
    items: Vec<HtmlCol>,
}

impl<I: IntoIterator<Item = HtmlCol>> From<I> for TableColumns {
    fn from(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl From<HtmlCol> for TableColumns {
    fn from(item: HtmlCol) -> Self {
        Self { items: vec![item] }
    }
}

/// Shorthand for `HtmlCol<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let col = col!().id("table_column");
///
/// assert_eq!(col.bake(),
/// r#"<col id="table_column" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let item = col!().class("item");
/// let description = col!().class("description");
///
/// let cols = bake_block![item, description];
///
/// assert_eq!(cols,
/// r#"<col class="item" />
/// <col class="description" />"#);
/// ```
#[macro_export]
macro_rules! col {
    () => {
        $crate::html::HtmlCol::<()>::new()
    };
}
