use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<colgroup>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/colgroup)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let colgroup = HtmlColgroup::new().id("table_column_group");
///
/// assert_eq!(
///     colgroup.bake(),
///     r#"<colgroup id="table_column_group"></colgroup>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let item = HtmlCol::new().class("item");
/// let description = HtmlCol::new().class("description");
///
/// let colgroup = HtmlColgroup::new().content([item, description]);
///
/// assert_eq!(
///     colgroup.bake(),
///     r#"<colgroup>
///     <col class="item" />
///     <col class="description" /></colgroup>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <colgroup
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</colgroup>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ColgroupRecipe, content = TableColumns)]
pub struct HtmlColgroup<R: ColgroupRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: ColgroupAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<span>` element specific attributes.
///
/// [MDN Documentation]()
///
/// # Askama template
///
/// ```askama
/// {{- span | bake_attr("span") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ColgroupAttrs {
    pub span: Option<u32>,
}

pub trait HasColgroupAttrs: Sized {
    fn colgroup_attrs_mut(&mut self) -> &mut ColgroupAttrs;

    /// Number of columns spanned by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/colgroup#span)
    fn span(mut self, value: u32) -> Self {
        self.colgroup_attrs_mut().span = Some(value);
        self
    }
}

impl HasColgroupAttrs for ColgroupAttrs {
    fn colgroup_attrs_mut(&mut self) -> &mut ColgroupAttrs {
        self
    }
}

impl HasColgroupAttrs for &mut ColgroupAttrs {
    fn colgroup_attrs_mut(&mut self) -> &mut ColgroupAttrs {
        self
    }
}

impl<R: ColgroupRecipe> HasColgroupAttrs for HtmlColgroup<R> {
    fn colgroup_attrs_mut(&mut self) -> &mut ColgroupAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlColgroup`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let colgroup = colgroup!().id("table_column_group");
///
/// assert_eq!(
///     colgroup.bake(),
///     r#"<colgroup id="table_column_group"></colgroup>"#
/// );
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
/// assert_eq!(
///     colgroup.bake(),
///     r#"<colgroup>
///     <col class="item" />
///     <col class="description" /></colgroup>"#
/// );
/// ```
#[macro_export]
macro_rules! colgroup {
    () => {
        $crate::html::HtmlColgroup::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlColgroup::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlColgroup::new().content([$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlColgroup::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlColgroup::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlColgroup::<$crate::cookbook_type!($($r),+)>::from_cookbook().content([$first $(, $rest)*])
    };
}
