use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// assert_eq!(
///     template.bake(),
///     r#"<template id="content_template"></template>"#
/// );
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
/// assert_eq!(
///     template.bake(),
///     r#"<template id="tmpl">
///   <p>Knock knock.</p>
///   <p>Who's there?</p>
///   <p><slot name="setup"></slot>.</p>
///   <p><slot name="setup"></slot> who?</p>
///   <p><slot name="punchline"></slot></p>
/// </template>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <template
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</template>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TemplateRecipe, content = Cow<'static, str>)]
pub struct HtmlTemplate<R: TemplateRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: TemplateAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<template>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- shadowrootmode | bake_attr("shadowrootmode") -}}
/// {{- shadowrootclonable | bake_bool_attr("shadowrootclonable") -}}
/// {{- shadowrootcustomelementregistry | bake_bool_attr("shadowrootcustomelementregistry") -}}
/// {{- shadowrootdelegatesfocus | bake_bool_attr("shadowrootdelegatesfocus") -}}
/// {{- shadowrootdelegatesfocus | bake_bool_attr("shadowrootdelegatesfocus") -}}
/// {{- shadowrootserializable | bake_bool_attr("shadowrootserializable") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TemplateAttrs {
    pub shadowrootmode: Option<Cow<'static, str>>,
    pub shadowrootclonable: bool,
    pub shadowrootcustomelementregistry: bool,
    pub shadowrootdelegatesfocus: bool,
    pub shadowrootserializable: bool,
}

pub trait HasTemplateAttrs: Sized {
    fn template_attrs_mut(&mut self) -> &mut TemplateAttrs;

    /// Sets clonable on a declarative shadow root.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootclonable)
    fn shadowrootclonable(mut self, value: bool) -> Self {
        self.template_attrs_mut().shadowrootclonable = value;
        self
    }

    /// Enables declarative shadow roots to indicate they will use a custom
    /// element registry.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootcustomelementregistry)
    fn shadowrootcustomelementregistry(mut self, value: bool) -> Self {
        self.template_attrs_mut().shadowrootcustomelementregistry = value;
        self
    }

    /// Sets delegates focus on a declarative shadow root.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootdelegatesfocus)
    fn shadowrootdelegatesfocus(mut self, value: bool) -> Self {
        self.template_attrs_mut().shadowrootdelegatesfocus = value;
        self
    }

    /// Enables streaming declarative shadow roots.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootmode)
    fn shadowrootmode(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.template_attrs_mut().shadowrootmode = Some(value.into());
        self
    }

    /// Sets serializable on a declarative shadow root.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootserializable)
    fn shadowrootserializable(mut self, value: bool) -> Self {
        self.template_attrs_mut().shadowrootserializable = value;
        self
    }

    // NOTE: Include `shadowrootreferencetarget` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template#shadowrootreferencetarget)
}

impl HasTemplateAttrs for TemplateAttrs {
    fn template_attrs_mut(&mut self) -> &mut TemplateAttrs {
        self
    }
}

impl HasTemplateAttrs for &mut TemplateAttrs {
    fn template_attrs_mut(&mut self) -> &mut TemplateAttrs {
        self
    }
}

impl<R: TemplateRecipe> HasTemplateAttrs for HtmlTemplate<R> {
    fn template_attrs_mut(&mut self) -> &mut TemplateAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlTemplate`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let template = template!().id("content_template");
///
/// assert_eq!(
///     template.bake(),
///     r#"<template id="content_template"></template>"#
/// );
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
