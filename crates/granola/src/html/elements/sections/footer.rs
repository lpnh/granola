use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<footer>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/footer)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let footer = HtmlFooter::new().id("footer");
///
/// assert_eq!(footer.bake(), r#"<footer id="footer"></footer>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let content = HtmlSmall::new().content("&copy; 2026 Oats &amp; Ends Café");
/// let paragraph = HtmlP::new().content(content);
///
/// let footer = HtmlFooter::new().content(paragraph);
///
/// assert_eq!(
///     footer.bake(),
///     r#"<footer><p><small>&copy; 2026 Oats &amp; Ends Café</small></p></footer>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <footer
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</footer>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = FooterRecipe, content = Bake)]
pub struct HtmlFooter<R: FooterRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// group, presentation or none
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlFooter`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let footer = footer!().id("footer");
///
/// assert_eq!(footer.bake(), r#"<footer id="footer"></footer>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let content = small!("&copy; 2026 Oats &amp; Ends Café");
/// let paragraph = p!(content);
///
/// let footer = footer!(paragraph);
///
/// assert_eq!(
///     footer.bake(),
///     r#"<footer><p><small>&copy; 2026 Oats &amp; Ends Café</small></p></footer>"#
/// );
/// ```
#[macro_export]
macro_rules! footer {
    () => {
        $crate::html::HtmlFooter::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlFooter::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlFooter::new().content($crate::bake![$first $(, $rest)*])
    };

}
