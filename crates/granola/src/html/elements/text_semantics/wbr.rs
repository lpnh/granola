use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::prelude::*;

/// The HTML `<wbr />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/wbr)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let wbr: HtmlWbr = HtmlWbr::new().id("line_break_opportunity");
///
/// assert_eq!(wbr.bake(), r#"<wbr id="line_break_opportunity" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <wbr
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = WbrRecipe)]
pub struct HtmlWbr<R: WbrRecipe = ()> {
    _recipe: PhantomData<R>,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<R: WbrRecipe> HtmlWbr<R> {
    pub fn new() -> Self {
        Self::from_cookbook()
    }
}

/// Shorthand for `HtmlWbr`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let wbr = wbr!().id("line_break_opportunity");
///
/// assert_eq!(wbr.bake(), r#"<wbr id="line_break_opportunity" />"#);
/// ```
#[macro_export]
macro_rules! wbr {
    () => {
        $crate::html::HtmlWbr::<()>::new()
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlAbbr::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
}
