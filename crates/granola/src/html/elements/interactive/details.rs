use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<details>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let details = HtmlDetails::new().id("details_disclosure");
///
/// assert_eq!(
///     details.bake(),
///     r#"<details id="details_disclosure"></details>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let summary = HtmlSummary::new().content("Pandora's box");
///
/// let details = HtmlDetails::new().fold_in(summary).fold_in("Hope");
///
/// assert_eq!(
///     details.bake(),
///     r#"<details><summary>Pandora's box</summary>Hope</details>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <details
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</details>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DetailsRecipe, content = Cow<'static, str>)]
pub struct HtmlDetails<R: DetailsRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: DetailsAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<details>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- name | bake_attr("name") -}}
/// {{- open | bake_bool_attr("open") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct DetailsAttrs {
    pub name: Option<Cow<'static, str>>,
    pub open: bool,
}

pub trait HasDetailsAttrs: Sized {
    fn details_attrs_mut(&mut self) -> &mut DetailsAttrs;

    /// Name of group of mutually-exclusive details elements.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.details_attrs_mut().name = Some(value.into());
        self
    }

    /// Whether the details are visible.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details#open)
    fn open(mut self, value: bool) -> Self {
        self.details_attrs_mut().open = value;
        self
    }
}

impl HasDetailsAttrs for DetailsAttrs {
    fn details_attrs_mut(&mut self) -> &mut DetailsAttrs {
        self
    }
}

impl HasDetailsAttrs for &mut DetailsAttrs {
    fn details_attrs_mut(&mut self) -> &mut DetailsAttrs {
        self
    }
}

impl<R: DetailsRecipe> HasDetailsAttrs for HtmlDetails<R> {
    fn details_attrs_mut(&mut self) -> &mut DetailsAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlDetails`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let details = details!().id("details_disclosure");
///
/// assert_eq!(
///     details.bake(),
///     r#"<details id="details_disclosure"></details>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let summary = summary!("Pandora's box");
///
/// let details = details![summary, "Hope"];
///
/// assert_eq!(
///     details.bake(),
///     r#"<details><summary>Pandora's box</summary>Hope</details>"#
/// );
/// ```
#[macro_export]
macro_rules! details {
    () => {
        $crate::html::HtmlDetails::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDetails::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDetails::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlDetails::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlDetails::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDetails::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
