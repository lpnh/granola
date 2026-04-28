use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait VarTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl VarTag for () {}

/// The HTML `<var>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/var)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let var: HtmlVar = HtmlVar::empty().id("variable");
///
/// assert_eq!(var.bake(),
/// r#"<var id="variable"></var>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let var: HtmlVar = HtmlVar::new("a");
///
/// let triangle = bake_inline!["An equilateral triangle with side ", var];
///
/// assert_eq!(triangle,
/// r#"An equilateral triangle with side <var>a</var>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <var
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</var>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlVar<M: VarTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: VarTag> HtmlVar<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }
}

/// Shorthand for `HtmlVar<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let var = var!().id("variable");
///
/// assert_eq!(var.bake(),
/// r#"<var id="variable"></var>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let var = var!("a");
///
/// let triangle = bake_inline!["An equilateral triangle with side ", var];
///
/// assert_eq!(triangle,
/// r#"An equilateral triangle with side <var>a</var>"#);
/// ```
#[macro_export]
macro_rules! var {
    () => {
        $crate::html::HtmlVar::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlVar::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlVar::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlVar::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlVar::<()>::new($crate::bake_inline![$($content),+])
    };
}
