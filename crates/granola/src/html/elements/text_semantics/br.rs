use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::prelude::*;

/// The HTML `<br />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/br)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let br = HtmlBr::new().id("line_break");
///
/// assert_eq!(br.bake(), r#"<br id="line_break" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let br = HtmlBr::new();
///
/// let roses = "Roses are red,";
/// let violets = "Violets are blue.";
///
/// let poem = HtmlP::new().content(bake![roses, br, violets]);
///
/// assert_eq!(
///     poem.bake(),
///     r#"<p>Roses are red,<br />Violets are blue.</p>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <br
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BrRecipe)]
pub struct HtmlBr<R: BrRecipe = ()> {
    _recipe: PhantomData<R>,
    /// # Permitted ARIA roles
    ///
    /// none, presentation
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlBr`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let br = br!().id("line_break");
///
/// assert_eq!(br.bake(), r#"<br id="line_break" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let br = br!();
///
/// let poem = p!("Roses are red,", br!(), "Violets are blue.");
///
/// assert_eq!(
///     poem.bake(),
///     r#"<p>Roses are red,<br />Violets are blue.</p>"#
/// );
/// ```
#[macro_export]
macro_rules! br {
    () => {
        $crate::html::HtmlBr::new()
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlBr::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
}
