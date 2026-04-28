use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait TemplateTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl TemplateTag for () {}

/// The HTML `<template>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let template: HtmlTemplate = HtmlTemplate::empty().id("content_template");
///
/// assert_eq!(template.bake(),
/// r#"<template id="content_template"></template>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let knock_knock: HtmlP = HtmlP::new("Knock knock.");
/// let who_s_there: HtmlP = HtmlP::new("Who's there?");
///
/// let name_slot: HtmlSlot = HtmlSlot::empty().name("setup");
/// let name_p1: HtmlP = HtmlP::new(bake_inline![name_slot, "."]);
/// let name_p2: HtmlP = HtmlP::new(bake_inline![name_slot, " who?"]);
///
/// let punchline_slot: HtmlSlot = HtmlSlot::empty().name("punchline");
/// let punchline: HtmlP = HtmlP::new(punchline_slot);
///
/// let content = bake_block![knock_knock, who_s_there, name_p1, name_p2, punchline];
///
/// let template: HtmlTemplate = HtmlTemplate::new(content).id("tmpl");
///
/// assert_eq!(template.bake(),
/// r#"<template id="tmpl">
///   <p>Knock knock.</p>
///   <p>Who's there?</p>
///   <p><slot name="setup"></slot>.</p>
///   <p><slot name="setup"></slot> who?</p>
///   <p><slot name="punchline"></slot></p>
/// </template>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <template
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</template>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTemplate<M: TemplateTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TemplateTag> HtmlTemplate<M> {
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

    /// Sets clonable on a declarative shadow root.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootclonable)
    pub fn shadowrootclonable(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("shadowrootclonable");
        }
        self
    }

    /// Enables declarative shadow roots to indicate they will use a custom element registry.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootcustomelementregistry)
    pub fn shadowrootcustomelementregistry(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self
                .specific_attrs
                .add_bool_attr("shadowrootcustomelementregistry");
        }
        self
    }

    /// Sets delegates focus on a declarative shadow root.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootdelegatesfocus)
    pub fn shadowrootdelegatesfocus(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self
                .specific_attrs
                .add_bool_attr("shadowrootdelegatesfocus");
        }
        self
    }

    /// Enables streaming declarative shadow roots.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootmode)
    pub fn shadowrootmode(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("shadowrootmode", value);
        self
    }

    /// Sets serializable on a declarative shadow root.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootserializable)
    pub fn shadowrootserializable(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("shadowrootserializable");
        }
        self
    }

    // NOTE: Include `shadowrootreferencetarget` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootreferencetarget)
}

/// Shorthand for `HtmlTemplate<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let template = template!().id("content_template");
///
/// assert_eq!(template.bake(),
/// r#"<template id="content_template"></template>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let knock_knock = p!("Knock knock.");
/// let who_s_there = p!("Who's there?");
///
/// let name_slot = slot!().name("setup");
/// let name_p1 = p!(@inline name_slot, ".");
/// let name_p2 = p!(@inline name_slot, " who?");
///
/// let punchline_slot = slot!().name("punchline");
/// let punchline = p!(punchline_slot);
///
/// let content = bake_block![
///     knock_knock,
///     who_s_there,
///     name_p1,
///     name_p2,
///     punchline
/// ];
///
/// let template = template!(content).id("tmpl");
///
/// assert_eq!(template.bake(),
/// r#"<template id="tmpl">
///   <p>Knock knock.</p>
///   <p>Who's there?</p>
///   <p><slot name="setup"></slot>.</p>
///   <p><slot name="setup"></slot> who?</p>
///   <p><slot name="punchline"></slot></p>
/// </template>"#);
/// ```
#[macro_export]
macro_rules! template {
    () => {
        $crate::html::HtmlTemplate::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTemplate::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTemplate::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTemplate::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTemplate::<()>::new($crate::bake_inline![$($content),+])
    };
}
