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
/// let section = HtmlSection::new().id("generic_section");
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
/// let h2 = HtmlH2::new().content("Latest news");
/// let ul = HtmlUl::new().content(HtmlLi::new().content("New café on Oak Street"));
///
/// let section = HtmlSection::new().fold_in(h2).fold_in(ul);
///
/// assert_eq!(
///     section.bake(),
///     r#"<section><h2>Latest news</h2><ul><li>New café on Oak Street</li></ul></section>"#
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
/// >{{ content | kirei }}</section>
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

impl<R: SectionRecipe<Content = Cow<'static, str>>> HtmlSection<R> {
    pub fn fold_in(mut self, content: impl Into<Cow<'static, str>>) -> Self {
        FoldIn::fold_in(&mut self.content, content.into());
        self
    }
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
///     r#"<section><h2>Latest news</h2><ul><li>New café on Oak Street</li></ul></section>"#
/// );
/// ```
#[macro_export]
macro_rules! section {
    () => {
        $crate::html::HtmlSection::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlSection::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlSection::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlSection::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; $content:expr $(,)?) => {
        $crate::html::HtmlSection::<$r>::from_cookbook().content($content)
    };
    (@cookbook $r:ty ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlSection::<$r>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
