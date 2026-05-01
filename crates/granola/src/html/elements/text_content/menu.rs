use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// directory, group, listbox, menu, menubar, none, presentation,
///     radiogroup, tablist, toolbar or tree
pub trait MenuTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = ListItems;

    fn recipe(element: HtmlMenu<Self>) -> HtmlMenu<Self> {
        element
    }
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
        let element = Self {
            content: content.into(),
            ..Default::default()
        };

        M::recipe(element)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }
}

/// Shorthand for `HtmlMenu<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let menu = menu!().id("menu");
///
/// assert_eq!(menu.bake(),
/// r#"<menu id="menu"></menu>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let items = [
///   li!("Buy"),
///   li!("Use"),
///   li!("Break"),
///   li!("Fix"),
/// ];
///
/// let technologic = menu!(items);
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
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let technologic = menu![
///   li!("Write"),
///   li!("Cut"),
///   li!("Paste"),
///   li!("Save"),
/// ];
///
/// assert_eq!(technologic.bake(),
/// r#"<menu>
///   <li>Write</li>
///   <li>Cut</li>
///   <li>Paste</li>
///   <li>Save</li>
/// </menu>"#);
/// ```
#[macro_export]
macro_rules! menu {
    () => {
        $crate::html::HtmlMenu::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlMenu::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlMenu::<()>::new([$first $(, $rest)*])
    };
}
