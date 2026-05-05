use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- attrs -}}
///   {{- specific_attrs -}}
/// >{{ content | kirei(2) }}</slot>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SlotTag, content = Cow<'static, str>, specific = SlotAttrs)]
pub struct HtmlSlot<M: SlotTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub attrs: Attrs,
    pub specific_attrs: SlotAttrs,
}

/// The HTML `<slot>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/slot#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- name | bake_attr("name") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct SlotAttrs {
    pub name: Option<Cow<'static, str>>,
}

pub trait HasSlotAttrs: Sized {
    fn slot_attrs_mut(&mut self) -> &mut SlotAttrs;

    /// Name of shadow tree slot.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/slot#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.slot_attrs_mut().name = Some(value.into());
        self
    }
}

impl HasSlotAttrs for SlotAttrs {
    fn slot_attrs_mut(&mut self) -> &mut SlotAttrs {
        self
    }
}

impl HasSlotAttrs for &mut SlotAttrs {
    fn slot_attrs_mut(&mut self) -> &mut SlotAttrs {
        self
    }
}

impl<M: SlotTag> HasSlotAttrs for HtmlSlot<M> {
    fn slot_attrs_mut(&mut self) -> &mut SlotAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlSlot`.
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
