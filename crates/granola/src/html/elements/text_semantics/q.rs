use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<q>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let q: HtmlQ = HtmlQ::empty().id("inline_quotation");
///
/// assert_eq!(q.bake(),
/// r#"<q id="inline_quotation"></q>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let q: HtmlQ = HtmlQ::new(bake_newline!("This element is intended for short quotations"))
///     .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q");
///
/// assert_eq!(q.bake(),
/// r#"<q cite="https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q">
///   This element is intended for short quotations
/// </q>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <q
///   {{- attrs -}}
///   {{- specific_attrs -}}
/// >{{ content | kirei(2) }}</q>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = QTag, content = Cow<'static, str>)]
pub struct HtmlQ<M: QTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
    pub specific_attrs: QAttrs,
}

/// The HTML `<q>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- cite | bake_attr("cite") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct QAttrs {
    pub cite: Option<Cow<'static, str>>,
}

pub trait HasQAttrs: Sized {
    fn q_attrs_mut(&mut self) -> &mut QAttrs;

    /// Link to the source of the quotation or more information about the edit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q#cite)
    fn cite(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.q_attrs_mut().cite = Some(value.into());
        self
    }
}

impl HasQAttrs for QAttrs {
    fn q_attrs_mut(&mut self) -> &mut QAttrs {
        self
    }
}

impl HasQAttrs for &mut QAttrs {
    fn q_attrs_mut(&mut self) -> &mut QAttrs {
        self
    }
}

impl<M: QTag> HasQAttrs for HtmlQ<M> {
    fn q_attrs_mut(&mut self) -> &mut QAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlQ`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let q = q!().id("inline_quotation");
///
/// assert_eq!(q.bake(),
/// r#"<q id="inline_quotation"></q>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let q = q!(@newline "This element is intended for short quotations")
///     .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q");
///
/// assert_eq!(q.bake(),
/// r#"<q cite="https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q">
///   This element is intended for short quotations
/// </q>"#);
/// ```
#[macro_export]
macro_rules! q {
    () => {
        $crate::html::HtmlQ::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlQ::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlQ::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlQ::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlQ::<()>::new($crate::bake_inline![$($content),+])
    };
}
