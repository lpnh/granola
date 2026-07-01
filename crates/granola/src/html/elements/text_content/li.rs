use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, oven::FoldIn, prelude::*};

/// The HTML `<li>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/li)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let li = HtmlLi::new().id("list_item");
///
/// assert_eq!(li.bake(), r#"<li id="list_item"></li>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let sugar = HtmlLi::new().content("sugar");
/// let spice = HtmlLi::new().content("spice");
///
/// let items = bake![sugar, spice];
///
/// assert_eq!(items, r#"<li>sugar</li><li>spice</li>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <li
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</li>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = LiRecipe, content = Cow<'static, str>)]
pub struct HtmlLi<R: LiRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// menuitem, menuitemcheckbox, menuitemradio, option, none, presentation,
    ///     radio, separator, tab, treeitem
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: LiAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<li>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/li#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- value | bake_attr("value") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct LiAttrs {
    value: Option<u32>,
}

pub trait HasLiAttrs: Sized {
    fn li_attrs_mut(&mut self) -> &mut LiAttrs;

    /// Ordinal value of the list item.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/li#value)
    fn value(mut self, value: u32) -> Self {
        self.li_attrs_mut().value = Some(value);
        self
    }
}

impl HasLiAttrs for LiAttrs {
    fn li_attrs_mut(&mut self) -> &mut LiAttrs {
        self
    }
}

impl HasLiAttrs for &mut LiAttrs {
    fn li_attrs_mut(&mut self) -> &mut LiAttrs {
        self
    }
}

impl<R: LiRecipe> HasLiAttrs for HtmlLi<R> {
    fn li_attrs_mut(&mut self) -> &mut LiAttrs {
        &mut self.specific_attrs
    }
}

/// A collection of HTML `<li>` items.
///
/// The content of [`HtmlMenu`], [`HtmlOl`], or [`HtmlUl`].
///
/// ```askama
/// {%- for li in items -%}
///     {{ li }}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
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

impl Extend<HtmlLi> for ListItems {
    fn extend<I: IntoIterator<Item = HtmlLi>>(&mut self, iter: I) {
        self.items.extend(iter);
    }
}

impl FoldIn for ListItems {
    fn fold_in(&mut self, mut content: Self) {
        if self.items.is_empty() {
            self.items = content.items;
        } else {
            self.items.append(&mut content.items);
        }
    }
}

/// Shorthand for `HtmlLi`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let li = li!().id("list_item");
///
/// assert_eq!(li.bake(), r#"<li id="list_item"></li>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sugar = li!("sugar");
/// let spice = li!("spice");
///
/// let items = bake![sugar, spice];
///
/// assert_eq!(items, r#"<li>sugar</li><li>spice</li>"#);
/// ```
#[macro_export]
macro_rules! li {
    () => {
        $crate::html::HtmlLi::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlLi::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlLi::new().content($crate::bake![$first $(, $rest)*])
    };

}
