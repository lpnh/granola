use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait HgroupTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl HgroupTag for () {}

/// The HTML `<hgroup>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hgroup)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let hgroup: HtmlHgroup = HtmlHgroup::empty().id("heading_group");
///
/// assert_eq!(hgroup.bake(),
/// r#"<hgroup id="heading_group"></hgroup>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let heading: HtmlH2 = HtmlH2::new("Capítulo VIII.");
///
/// let subtitle = "Del buen suceso que el valeroso don Quijote tuvo en la espantable y
/// jamás imaginada aventura de los molinos de viento, con otros sucesos
/// dignos de felice recordación";
///
/// let paragraph: HtmlP = HtmlP::new(subtitle);
///
/// let hgroup: HtmlHgroup = HtmlHgroup::new(bake_block![heading, paragraph]);
///
/// assert_eq!(hgroup.bake(),
/// r#"<hgroup>
///   <h2>Capítulo VIII.</h2>
///   <p>
///     Del buen suceso que el valeroso don Quijote tuvo en la espantable y
///     jamás imaginada aventura de los molinos de viento, con otros sucesos
///     dignos de felice recordación
///   </p>
/// </hgroup>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <hgroup
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</hgroup>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlHgroup<M: HgroupTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: HgroupTag> HtmlHgroup<M> {
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

/// Shorthand for `HtmlHgroup<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let hgroup = hgroup!().id("heading_group");
///
/// assert_eq!(hgroup.bake(),
/// r#"<hgroup id="heading_group"></hgroup>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let heading = h2!("Capítulo VIII.");
///
/// let subtitle = "Del buen suceso que el valeroso don Quijote tuvo en la espantable y
/// jamás imaginada aventura de los molinos de viento, con otros sucesos
/// dignos de felice recordación";
///
/// let paragraph = p!(subtitle);
///
/// let hgroup = hgroup!(heading, paragraph);
///
/// assert_eq!(hgroup.bake(),
/// r#"<hgroup>
///   <h2>Capítulo VIII.</h2>
///   <p>
///     Del buen suceso que el valeroso don Quijote tuvo en la espantable y
///     jamás imaginada aventura de los molinos de viento, con otros sucesos
///     dignos de felice recordación
///   </p>
/// </hgroup>"#);
/// ```
#[macro_export]
macro_rules! hgroup {
    () => {
        $crate::html::HtmlHgroup::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlHgroup::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlHgroup::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlHgroup::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlHgroup::<()>::new($crate::bake_inline![$($content),+])
    };
}
