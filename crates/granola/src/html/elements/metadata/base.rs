use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<base />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let base: HtmlBase = HtmlBase::empty().id("document_base_url");
///
/// assert_eq!(base.bake(),
/// r#"<base id="document_base_url" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let base: HtmlBase = HtmlBase::new("https://www.example.com");
///
/// assert_eq!(base.bake(),
/// r#"<base href="https://www.example.com" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <base
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BaseTag, attrs = BaseAttrs)]
pub struct HtmlBase<M: BaseTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: BaseAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<M: BaseTag> HtmlBase<M> {
    pub fn new(href: impl Into<Cow<'static, str>>) -> Self {
        let mut global_attrs = GlobalAttrs::default();
        M::global_attrs_recipe(&mut global_attrs);

        let mut specific_attrs = BaseAttrs::default().href(href);
        M::specific_attrs_recipe(&mut specific_attrs);

        let mut global_aria_attrs = GlobalAriaAttrs::default();
        M::global_aria_attrs_recipe(&mut global_aria_attrs);

        let mut custom_data_attrs = CustomDataAttrs::default();
        M::custom_data_attrs_recipe(&mut custom_data_attrs);

        let mut event_handlers = EventHandlers::default();
        M::event_handlers_recipe(&mut event_handlers);

        Self {
            global_attrs,
            specific_attrs,
            global_aria_attrs,
            custom_data_attrs,
            event_handlers,
            ..Default::default()
        }
    }
}

/// The HTML `<todo>` element specific attributes.
///
/// [MDN Documentation]()
///
/// # Askama template
///
/// ```askama
/// {{- href | bake_attr("href") -}}
/// {{- target | bake_attr("target") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct BaseAttrs {
    pub href: Option<Cow<'static, str>>,
    pub target: Option<Cow<'static, str>>,
}

pub trait HasBaseAttrs: Sized {
    fn base_attrs_mut(&mut self) -> &mut BaseAttrs;

    /// Document base URL.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base#href)
    fn href(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.base_attrs_mut().href = Some(value.into());
        self
    }

    /// Default navigable for hyperlink navigation and form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base#target)
    fn target(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.base_attrs_mut().target = Some(value.into());
        self
    }
}

impl HasBaseAttrs for BaseAttrs {
    fn base_attrs_mut(&mut self) -> &mut BaseAttrs {
        self
    }
}

impl HasBaseAttrs for &mut BaseAttrs {
    fn base_attrs_mut(&mut self) -> &mut BaseAttrs {
        self
    }
}

impl<M: BaseTag> HasBaseAttrs for HtmlBase<M> {
    fn base_attrs_mut(&mut self) -> &mut BaseAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlBase`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let base = base!().id("document_base_url");
///
/// assert_eq!(base.bake(),
/// r#"<base id="document_base_url" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let base = base!("https://www.example.com");
///
/// assert_eq!(base.bake(),
/// r#"<base href="https://www.example.com" />"#);
/// ```
#[macro_export]
macro_rules! base {
    () => {
        $crate::html::HtmlBase::<()>::empty()
    };
    ($href: expr $(,)?) => {
        $crate::html::HtmlBase::<()>::new($href)
    };
}
