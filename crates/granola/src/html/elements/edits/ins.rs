use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// r#"<ins datetime="2016-11-10" cite="https://blog.rust-lang.org/2016/11/10/Rust-1.13">
///   ?
/// </ins>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <ins
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</ins>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = InsTag, content = Cow<'static, str>)]
pub struct HtmlIns<R: InsTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: InsAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<ins>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- datetime | bake_attr("datetime") -}}
/// {{- cite | bake_attr("cite") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct InsAttrs {
    pub datetime: Option<Cow<'static, str>>,
    pub cite: Option<Cow<'static, str>>,
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

impl<R: InsTag> HasInsAttrs for HtmlIns<R> {
    fn ins_attrs_mut(&mut self) -> &mut InsAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlIns`.
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
/// r#"<ins datetime="2016-11-10" cite="https://blog.rust-lang.org/2016/11/10/Rust-1.13">
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
