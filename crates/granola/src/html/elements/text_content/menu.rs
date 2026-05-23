use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</menu>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = MenuTag, content = ListItems)]
pub struct HtmlMenu<R: MenuTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// directory, group, listbox, menu, menubar, none, presentation,
    ///     radiogroup, tablist, toolbar or tree
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlMenu`.
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
