use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<del>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let del = HtmlDel::new().id("deleted_text");
///
/// assert_eq!(del.bake(), r#"<del id="deleted_text"></del>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let del = HtmlDel::new()
///     .content(bake_newline!("try!"))
///     .datetime("2019-11-07")
///     .cite("https://github.com/rust-lang/rust/pull/62672");
///
/// assert_eq!(
///     del.bake(),
///     r#"<del datetime="2019-11-07" cite="https://github.com/rust-lang/rust/pull/62672">
///   try!
/// </del>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <del
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</del>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DelRecipe, content = Cow<'static, str>)]
pub struct HtmlDel<R: DelRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: DelAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<del>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- datetime | bake_attr("datetime") -}}
/// {{- cite | bake_attr("cite") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct DelAttrs {
    pub datetime: Option<Cow<'static, str>>,
    pub cite: Option<Cow<'static, str>>,
}

pub trait HasDelAttrs: Sized {
    fn del_attrs_mut(&mut self) -> &mut DelAttrs;

    /// Link to the source of the quotation or more information about the edit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del#cite)
    fn cite(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.del_attrs_mut().cite = Some(value.into());
        self
    }

    /// Date and (optionally) time of the change.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del#datetime)
    fn datetime(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.del_attrs_mut().datetime = Some(value.into());
        self
    }
}

impl HasDelAttrs for DelAttrs {
    fn del_attrs_mut(&mut self) -> &mut DelAttrs {
        self
    }
}

impl HasDelAttrs for &mut DelAttrs {
    fn del_attrs_mut(&mut self) -> &mut DelAttrs {
        self
    }
}

impl<R: DelRecipe> HasDelAttrs for HtmlDel<R> {
    fn del_attrs_mut(&mut self) -> &mut DelAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlDel`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let del = del!().id("deleted_text");
///
/// assert_eq!(del.bake(), r#"<del id="deleted_text"></del>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let del = del!(@newline "try!")
///     .datetime("2019-11-07")
///     .cite("https://github.com/rust-lang/rust/pull/62672");
///
/// assert_eq!(del.bake(),
/// r#"<del datetime="2019-11-07" cite="https://github.com/rust-lang/rust/pull/62672">
///   try!
/// </del>"#);
/// ```
#[macro_export]
macro_rules! del {
    () => {
        $crate::html::HtmlDel::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDel::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDel::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content:expr $(,)?) => {
        $crate::html::HtmlDel::new().content($crate::bake_newline!($content))
    };
    (@inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlDel::new().content($crate::bake_inline![$($content),+])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlDel::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlDel::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDel::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlDel::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlDel::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_inline![$($content),+])
    };
}
