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
/// let q = HtmlQ::new().id("inline_quotation");
///
/// assert_eq!(q.bake(), r#"<q id="inline_quotation"></q>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let q = HtmlQ::new()
///     .content("This element is intended for short quotations")
///     .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q");
///
/// assert_eq!(
///     q.bake_pretty(),
///     r#"<q cite="https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q"
/// >This element is intended for short quotations</q>
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <q
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</q>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = QRecipe, content = Cow<'static, str>)]
pub struct HtmlQ<R: QRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: QAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
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

impl<R: QRecipe> HasQAttrs for HtmlQ<R> {
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
/// assert_eq!(q.bake(), r#"<q id="inline_quotation"></q>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let q = q!("This element is intended for short quotations")
///     .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q");
///
/// assert_eq!(
///     q.bake_pretty(),
///     r#"<q cite="https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q"
/// >This element is intended for short quotations</q>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! q {
    () => {
        $crate::html::HtmlQ::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlQ::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlQ::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlQ::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; $content:expr $(,)?) => {
        $crate::html::HtmlQ::<$r>::from_cookbook().content($content)
    };
    (@cookbook $r:ty ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlQ::<$r>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
