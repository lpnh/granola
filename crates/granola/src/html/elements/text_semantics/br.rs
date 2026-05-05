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
/// <br{{ attrs }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BrTag)]
pub struct HtmlBr<M: BrTag = ()> {
    _marker: PhantomData<M>,
    /// # Permitted ARIA roles
    ///
    /// none, presentation
    pub attrs: Attrs,
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
