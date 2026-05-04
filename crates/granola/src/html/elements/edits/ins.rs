use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

// # Permitted ARIA roles
//
// any

/// The HTML `<ins>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let ins: HtmlIns = HtmlIns::empty().id("inserted_text");
///
/// assert_eq!(ins.bake(),
/// r#"<ins id="inserted_text"></ins>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let ins: HtmlIns = HtmlIns::new(bake_newline!("?"))
///     .datetime("2016-11-10")
///     .cite("https://blog.rust-lang.org/2016/11/10/Rust-1.13");
///
/// assert_eq!(ins.bake(),
/// r#"<ins cite="https://blog.rust-lang.org/2016/11/10/Rust-1.13" datetime="2016-11-10">
///   ?
/// </ins>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <ins
///   {{- attrs -}}
///   {{- specific_attrs -}}
/// >{{ content | kirei(2) }}</ins>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = InsTag, content = Cow<'static, str>, specific = InsAttrs)]
pub struct HtmlIns<M: InsTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub attrs: Attrs,
    pub specific_attrs: InsAttrs,
}

/// # Askama template
///
/// ```askama
/// {{- cite | bake_attr("cite") -}}
/// {{- datetime | bake_attr("datetime") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct InsAttrs {
    pub cite: Option<Cow<'static, str>>,
    pub datetime: Option<Cow<'static, str>>,
}

pub trait HasInsAttrs: Sized {
    fn ins_attrs_mut(&mut self) -> &mut InsAttrs;

    /// Link to the source of the quotation or more information about the edit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins#cite)
    fn cite(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.ins_attrs_mut().cite = Some(value.into());
        self
    }

    /// Date and (optionally) time of the change.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins#datetime)
    fn datetime(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.ins_attrs_mut().datetime = Some(value.into());
        self
    }
}

impl HasInsAttrs for InsAttrs {
    fn ins_attrs_mut(&mut self) -> &mut InsAttrs {
        self
    }
}

impl HasInsAttrs for &mut InsAttrs {
    fn ins_attrs_mut(&mut self) -> &mut InsAttrs {
        self
    }
}

impl<M: InsTag> HasInsAttrs for HtmlIns<M> {
    fn ins_attrs_mut(&mut self) -> &mut InsAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlIns<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ins = ins!().id("inserted_text");
///
/// assert_eq!(ins.bake(),
/// r#"<ins id="inserted_text"></ins>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ins = ins!(@newline "?")
///     .datetime("2016-11-10")
///     .cite("https://blog.rust-lang.org/2016/11/10/Rust-1.13");
///
/// assert_eq!(ins.bake(),
/// r#"<ins cite="https://blog.rust-lang.org/2016/11/10/Rust-1.13" datetime="2016-11-10">
///   ?
/// </ins>"#);
/// ```
#[macro_export]
macro_rules! ins {
    () => {
        $crate::html::HtmlIns::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlIns::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlIns::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlIns::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlIns::<()>::new($crate::bake_inline![$($content),+])
    };

    (@recipe $($r:ty),+) => {
        $crate::html::HtmlIns::<$crate::rec!($($r),+)>::from_recipe()
    };
    (@recipe $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlIns::<$crate::rec!($($r),+)>::new($content)
    };
    (@recipe $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlIns::<$crate::rec!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@recipe $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlIns::<$crate::rec!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@recipe $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlIns::<$crate::rec!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
