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
/// let br: HtmlBr = HtmlBr::new().id("line_break");
///
/// assert_eq!(br.bake(),
/// r#"<br id="line_break" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let br: HtmlBr = HtmlBr::new();
///
/// let roses = bake_inline!["Roses are red,", br];
/// let violets = "Violets are blue.";
///
/// let poem: HtmlP = HtmlP::new(bake_block![roses, violets]);
///
/// assert_eq!(poem.bake(),
/// r#"<p>
///   Roses are red,<br />
///   Violets are blue.
/// </p>"#);
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
#[recipe(name = BrTag)]
pub struct HtmlBr<R: BrTag = ()> {
    _recipe: PhantomData<R>,
    /// # Permitted ARIA roles
    ///
    /// none, presentation
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<R: BrTag> HtmlBr<R> {
    pub fn new() -> Self {
        Self::from_recipe()
    }
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
/// assert_eq!(br.bake(),
/// r#"<br id="line_break" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let poem = p![
///     bake_inline!["Roses are red,", br!()],
///     "Violets are blue.",
/// ];
///
/// assert_eq!(poem.bake(),
/// r#"<p>
///   Roses are red,<br />
///   Violets are blue.
/// </p>"#);
/// ```
#[macro_export]
macro_rules! br {
    () => {
        $crate::html::HtmlBr::<()>::new()
    };

    (@recipe $($r:ty),+) => {
        $crate::html::HtmlAbbr::<$crate::rec!($($r),+)>::from_recipe()
    };
}
