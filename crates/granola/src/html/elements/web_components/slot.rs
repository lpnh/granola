use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait SlotTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl SlotTag for () {}

/// The HTML `<slot>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/slot)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let slot: HtmlSlot = HtmlSlot::empty().id("web_component_slot");
///
/// assert_eq!(slot.bake(),
/// r#"<slot id="web_component_slot"></slot>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let slot: HtmlSlot = HtmlSlot::empty().name("who");
///
/// assert_eq!(slot.bake(),
/// r#"<slot name="who"></slot>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <slot
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</slot>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSlot<M: SlotTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SlotTag> HtmlSlot<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    /// Name of shadow tree slot.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/slot#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }
}

/// Shorthand for `HtmlSlot<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let slot = slot!().id("web_component_slot");
///
/// assert_eq!(slot.bake(),
/// r#"<slot id="web_component_slot"></slot>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let slot = slot!().name("who");
///
/// assert_eq!(slot.bake(),
/// r#"<slot name="who"></slot>"#);
/// ```
#[macro_export]
macro_rules! slot {
    () => {
        $crate::html::HtmlSlot::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSlot::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSlot::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSlot::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSlot::<()>::new($crate::bake_inline![$($content),+])
    };
}
