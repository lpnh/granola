use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<col />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/col)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let col = HtmlCol::new().id("table_column");
///
/// assert_eq!(col.bake(), r#"<col id="table_column" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let item = HtmlCol::new().class("item");
/// let description = HtmlCol::new().class("description");
///
/// let cols = bake![item, description];
///
/// assert_eq!(cols, r#"<col class="item" /><col class="description" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <col
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ColRecipe)]
pub struct HtmlCol<R: ColRecipe = ()> {
    _recipe: PhantomData<R>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: ColAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<col>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/col#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- span | bake_attr("span") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ColAttrs {
    pub span: Option<u32>,
}

pub trait HasColAttrs: Sized {
    fn col_attrs_mut(&mut self) -> &mut ColAttrs;

    /// Number of columns spanned by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/col#span)
    fn span(mut self, value: u32) -> Self {
        self.col_attrs_mut().span = Some(value);
        self
    }
}

impl HasColAttrs for ColAttrs {
    fn col_attrs_mut(&mut self) -> &mut ColAttrs {
        self
    }
}

impl HasColAttrs for &mut ColAttrs {
    fn col_attrs_mut(&mut self) -> &mut ColAttrs {
        self
    }
}

impl<R: ColRecipe> HasColAttrs for HtmlCol<R> {
    fn col_attrs_mut(&mut self) -> &mut ColAttrs {
        &mut self.specific_attrs
    }
}

/// A collection of HTML `<col>` items.
///
/// The content of [`HtmlColgroup`].
///
/// ```askama
/// {%- for col in items %}
///     {{ col }}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
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

/// Shorthand for `HtmlCol`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let col = col!().id("table_column");
///
/// assert_eq!(col.bake(), r#"<col id="table_column" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let item = col!().class("item");
/// let description = col!().class("description");
///
/// let cols = bake![item, description];
///
/// assert_eq!(cols, r#"<col class="item" /><col class="description" />"#);
/// ```
#[macro_export]
macro_rules! col {
    () => {
        $crate::html::HtmlCol::new()
    };
    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlCol::<$r>::from_cookbook()
    };
}
