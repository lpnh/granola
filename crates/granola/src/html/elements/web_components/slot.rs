use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

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
/// let slot = HtmlSlot::new().id("web_component_slot");
///
/// assert_eq!(slot.bake(), r#"<slot id="web_component_slot"></slot>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let slot = HtmlSlot::new().name("who");
///
/// assert_eq!(slot.bake(), r#"<slot name="who"></slot>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <slot
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</slot>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SlotRecipe, content = Bake)]
pub struct HtmlSlot<R: SlotRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SlotAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
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
    pub name: Option<Bake>,
}

pub trait HasSlotAttrs: Sized {
    fn slot_attrs_mut(&mut self) -> &mut SlotAttrs;

    /// Name of shadow tree slot.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/slot#name)
    fn name(mut self, value: impl Into<Bake>) -> Self {
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

impl<R: SlotRecipe> HasSlotAttrs for HtmlSlot<R> {
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
/// assert_eq!(slot.bake(), r#"<slot id="web_component_slot"></slot>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let slot = slot!().name("who");
///
/// assert_eq!(slot.bake(), r#"<slot name="who"></slot>"#);
/// ```
#[macro_export]
macro_rules! slot {
    () => {
        $crate::html::HtmlSlot::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlSlot::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlSlot::new().content($crate::bake_ws![$first $(, $rest)*])
    };

}
