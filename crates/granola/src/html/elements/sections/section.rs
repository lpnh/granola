use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<section>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/section)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let section: HtmlSection = HtmlSection::empty().id("generic_section");
///
/// assert_eq!(
///     section.bake(),
///     r#"<section id="generic_section"></section>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let h2: HtmlH2 = HtmlH2::new("Latest news");
/// let ul: HtmlUl = HtmlUl::new(HtmlLi::new("New café on Oak Street"));
///
/// let section: HtmlSection = HtmlSection::new(bake_block![h2, ul]);
///
/// assert_eq!(
///     section.bake(),
///     r#"<section>
///   <h2>Latest news</h2>
///   <ul>
///     <li>New café on Oak Street</li>
///   </ul>
/// </section>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <section
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</section>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SectionRecipe, content = Cow<'static, str>)]
pub struct HtmlSection<R: SectionRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// alert, alertdialog, application, banner, complementary, contentinfo,
    /// dialog, document, feed, log, main, marquee, navigation, none, note,
    /// presentation, search, status, tabpanel
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlSection`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let section = section!().id("generic_section");
///
/// assert_eq!(
///     section.bake(),
///     r#"<section id="generic_section"></section>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h2 = h2!("Latest news");
/// let ul = ul!(li!("New café on Oak Street"));
///
/// let section = section!(h2, ul);
///
/// assert_eq!(
///     section.bake(),
///     r#"<section>
///   <h2>Latest news</h2>
///   <ul>
///     <li>New café on Oak Street</li>
///   </ul>
/// </section>"#
/// );
/// ```
#[macro_export]
macro_rules! section {
    () => {
        $crate::html::HtmlSection::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSection::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSection::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSection::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSection::<()>::new($crate::bake_inline![$($content),+])
    };
}
