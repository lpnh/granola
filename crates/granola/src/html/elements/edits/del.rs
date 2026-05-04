use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

// # Permitted ARIA roles
//
// any

/// The HTML `<del>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let del: HtmlDel = HtmlDel::empty().id("deleted_text");
///
/// assert_eq!(del.bake(),
/// r#"<del id="deleted_text"></del>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let del: HtmlDel = HtmlDel::new(bake_newline!("try!"))
///     .datetime("2019-11-07")
///     .cite("https://github.com/rust-lang/rust/pull/62672");
///
/// assert_eq!(del.bake(),
/// r#"<del cite="https://github.com/rust-lang/rust/pull/62672" datetime="2019-11-07">
///   try!
/// </del>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <del
///   {{- attrs -}}
///   {{- specific_attrs -}}
/// >{{ content | kirei(2) }}</del>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DelTag, content = Cow<'static, str>, specific = DelAttrs)]
pub struct HtmlDel<M: DelTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub attrs: Attrs,
    pub specific_attrs: DelAttrs,
}

/// # Askama template
///
/// ```askama
/// {{- cite | bake_attr("cite") -}}
/// {{- datetime | bake_attr("datetime") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct DelAttrs {
    pub cite: Option<Cow<'static, str>>,
    pub datetime: Option<Cow<'static, str>>,
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

impl<M: DelTag> HasDelAttrs for HtmlDel<M> {
    fn del_attrs_mut(&mut self) -> &mut DelAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlDel<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let del = del!().id("deleted_text");
///
/// assert_eq!(del.bake(),
/// r#"<del id="deleted_text"></del>"#);
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
        $crate::html::HtmlDel::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDel::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDel::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDel::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDel::<()>::new($crate::bake_inline![$($content),+])
    };

    (@recipe $($r:ty),+) => {
        $crate::html::HtmlDel::<$crate::rec!($($r),+)>::from_recipe()
    };
    (@recipe $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlDel::<$crate::rec!($($r),+)>::new($content)
    };
    (@recipe $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDel::<$crate::rec!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@recipe $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlDel::<$crate::rec!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@recipe $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlDel::<$crate::rec!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
