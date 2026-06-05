use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::prelude::*;

/// The HTML `<hr>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hr)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let hr = HtmlHr::new().id("thematic_break");
///
/// assert_eq!(hr.bake(), r#"<hr id="thematic_break" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let p1 = HtmlP::new().content("She blew out the candle. The room went dark.");
/// let p2 = HtmlP::new()
///     .content("Morning came with birds and the smell of bread from somewhere below.");
///
/// let hr = HtmlHr::new();
///
/// let story = bake_block![p1, "", hr, "", p2];
///
/// assert_eq!(
///     story,
///     r#"<p>She blew out the candle. The room went dark.</p>
///
/// <hr />
///
/// <p>Morning came with birds and the smell of bread from somewhere below.</p>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <hr
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = HrRecipe)]
pub struct HtmlHr<R: HrRecipe = ()> {
    _recipe: PhantomData<R>,
    /// # Permitted ARIA roles
    ///
    /// presentation or none
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlHr`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let hr = hr!().id("thematic_break");
///
/// assert_eq!(hr.bake(), r#"<hr id="thematic_break" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let p1 = p!("She blew out the candle. The room went dark.");
/// let p2 = p!("Morning came with birds and the smell of bread from somewhere below.");
///
/// let story = bake_block![p1, "", hr!(), "", p2];
///
/// assert_eq!(
///     story,
///     r#"<p>She blew out the candle. The room went dark.</p>
///
/// <hr />
///
/// <p>Morning came with birds and the smell of bread from somewhere below.</p>"#
/// );
/// ```
#[macro_export]
macro_rules! hr {
    () => {
        $crate::html::HtmlHr::new()
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlHr::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
}
