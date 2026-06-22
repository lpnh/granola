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
/// let menu = HtmlMenu::new().id("menu");
///
/// assert_eq!(menu.bake(), r#"<menu id="menu"></menu>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let items = [
///     HtmlLi::new().content("Buy"),
///     HtmlLi::new().content("Use"),
///     HtmlLi::new().content("Break"),
///     HtmlLi::new().content("Fix"),
/// ];
///
/// let technologic = HtmlMenu::new().content(items);
///
/// assert_eq!(
///     technologic.bake(),
///     r#"<menu><li>Buy</li><li>Use</li><li>Break</li><li>Fix</li></menu>"#
/// );
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
/// >{{ content | kirei }}</menu>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = MenuRecipe, content = ListItems)]
pub struct HtmlMenu<R: MenuRecipe = ()> {
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
/// assert_eq!(menu.bake(), r#"<menu id="menu"></menu>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let items = [li!("Buy"), li!("Use"), li!("Break"), li!("Fix")];
///
/// let technologic = menu!(items);
///
/// assert_eq!(
///     technologic.bake(),
///     r#"<menu><li>Buy</li><li>Use</li><li>Break</li><li>Fix</li></menu>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let technologic = menu![li!("Write"), li!("Cut"), li!("Paste"), li!("Save"),];
///
/// assert_eq!(
///     technologic.bake(),
///     r#"<menu><li>Write</li><li>Cut</li><li>Paste</li><li>Save</li></menu>"#
/// );
/// ```
#[macro_export]
macro_rules! menu {
    () => {
        $crate::html::HtmlMenu::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlMenu::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlMenu::new().content([$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlMenu::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlMenu::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlMenu::<$crate::cookbook_type!($($r),+)>::from_cookbook().content([$first $(, $rest)*])
    };
}
