use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// assert_eq!(ul.bake(), r#"<ul id="unordered_list"></ul>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let items = [
///     HtmlLi::new("sugar"),
///     HtmlLi::new("spice"),
///     HtmlLi::new("everything nice"),
/// ];
///
/// let ingredients: HtmlUl = HtmlUl::new(items);
///
/// assert_eq!(
///     ingredients.bake(),
///     r#"<ul>
///   <li>sugar</li>
///   <li>spice</li>
///   <li>everything nice</li>
/// </ul>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <ul
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</ul>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = UlTag, content = ListItems)]
pub struct HtmlUl<R: UlTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// directory, group, listbox, menu, menubar, none, presentation,
    /// radiogroup, tablist, toolbar, tree
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlUl`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ul = ul!().id("unordered_list");
///
/// assert_eq!(ul.bake(), r#"<ul id="unordered_list"></ul>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let items = [li!("sugar"), li!("spice"), li!("everything nice")];
///
/// let ingredients = ul!(items);
///
/// assert_eq!(
///     ingredients.bake(),
///     r#"<ul>
///   <li>sugar</li>
///   <li>spice</li>
///   <li>everything nice</li>
/// </ul>"#
/// );
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
/// assert_eq!(
///     ingredients.bake(),
///     r#"<ul>
///   <li>sugar</li>
///   <li>spice</li>
/// </ul>"#
/// );
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
