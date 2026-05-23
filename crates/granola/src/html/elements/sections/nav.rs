use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<nav>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/nav)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let nav: HtmlNav = HtmlNav::empty().id("navigation_section");
///
/// assert_eq!(nav.bake(),
/// r#"<nav id="navigation_section"></nav>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let location: HtmlA = HtmlA::new("Oak Street, corner of Elm Avenue").href("/location");
/// let menu: HtmlA = HtmlA::new("the menu").href("/menu");
/// let note: HtmlA = HtmlA::new("note").href("/contact");
///
/// let content = bake_block![
///     bake_inline!["You can find us at ", location, "."],
///     bake_inline!["Everything we make and love is on ", menu, "."],
///     bake_inline!["Have a thought? Send us a ", note, "."],
/// ];
///
/// let p: HtmlP = HtmlP::new(content);
/// let nav: HtmlNav = HtmlNav::new(p).aria_label("Site navigation");
///
/// assert_eq!(nav.bake(),
/// r#"<nav aria-label="Site navigation">
///   <p>
///     You can find us at <a href="/location">Oak Street, corner of Elm Avenue</a>.
///     Everything we make and love is on <a href="/menu">the menu</a>.
///     Have a thought? Send us a <a href="/contact">note</a>.
///   </p>
/// </nav>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <nav
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</nav>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = NavTag, content = Cow<'static, str>)]
pub struct HtmlNav<R: NavTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// menu, menubar, none, presentation, tablist
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlNav`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let nav = nav!().id("navigation_section");
///
/// assert_eq!(nav.bake(),
/// r#"<nav id="navigation_section"></nav>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let location = a!("Oak Street, corner of Elm Avenue").href("/location");
/// let menu = a!("the menu").href("/menu");
/// let note = a!("note").href("/contact");
///
/// let paragraph = p![
///     bake_inline!["You can find us at ", location, "."],
///     bake_inline!["Everything we make and love is on ", menu, "."],
///     bake_inline!["Have a thought? Send us a ", note, "."],
/// ];
///
/// let nav = nav!(paragraph).aria_label("Site navigation");
///
/// assert_eq!(nav.bake(),
/// r#"<nav aria-label="Site navigation">
///   <p>
///     You can find us at <a href="/location">Oak Street, corner of Elm Avenue</a>.
///     Everything we make and love is on <a href="/menu">the menu</a>.
///     Have a thought? Send us a <a href="/contact">note</a>.
///   </p>
/// </nav>"#);
/// ```
#[macro_export]
macro_rules! nav {
    () => {
        $crate::html::HtmlNav::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlNav::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlNav::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlNav::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlNav::<()>::new($crate::bake_inline![$($content),+])
    };
}
