use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait MarkTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl MarkTag for () {}

/// The HTML `<mark>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/mark)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let mark: HtmlMark = HtmlMark::empty().id("mark_text");
///
/// assert_eq!(mark.bake(),
/// r#"<mark id="mark_text"></mark>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let but_the_clouds: HtmlMark = HtmlMark::new("but the clouds");
///
/// let br: HtmlBr = HtmlBr::new();
///
/// let the_tower = bake_block![
///     bake_inline!["Seem ", but_the_clouds, " of the sky"],
///     br,
///     "When the horizon fades;",
///     br,
///     "Or a bird's sleepy cry",
///     br,
///     "Among the deepening shades."
/// ];
///
/// assert_eq!(the_tower,
/// r#"Seem <mark>but the clouds</mark> of the sky
/// <br />
/// When the horizon fades;
/// <br />
/// Or a bird's sleepy cry
/// <br />
/// Among the deepening shades."#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <mark
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</mark>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlMark<M: MarkTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: MarkTag> HtmlMark<M> {
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

/// Shorthand for `HtmlMark<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let mark = mark!().id("mark_text");
///
/// assert_eq!(mark.bake(),
/// r#"<mark id="mark_text"></mark>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let but_the_clouds = mark!("but the clouds");
///
/// let br = br!();
///
/// let the_tower = bake_block![
///     bake_inline!["Seem ", but_the_clouds, " of the sky"],
///     br,
///     "When the horizon fades;",
///     br,
///     "Or a bird's sleepy cry",
///     br,
///     "Among the deepening shades."
/// ];
///
/// assert_eq!(the_tower,
/// r#"Seem <mark>but the clouds</mark> of the sky
/// <br />
/// When the horizon fades;
/// <br />
/// Or a bird's sleepy cry
/// <br />
/// Among the deepening shades."#);
/// ```
#[macro_export]
macro_rules! mark {
    () => {
        $crate::html::HtmlMark::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlMark::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlMark::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlMark::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlMark::<()>::new($crate::bake_inline![$($content),+])
    };
}
