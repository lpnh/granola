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
/// let base = HtmlBase::new().id("document_base_url");
///
/// assert_eq!(base.bake(), r#"<base id="document_base_url" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let base = HtmlBase::new().href("https://www.example.com");
///
/// assert_eq!(base.bake(), r#"<base href="https://www.example.com" />"#);
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
#[recipe(name = BaseRecipe)]
pub struct HtmlBase<R: BaseRecipe = ()> {
    _recipe: PhantomData<R>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: BaseAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlBase {
    pub fn from_href(href: impl Into<Cow<'static, str>>) -> Self {
        Self::new().href(href)
    }
}

/// The HTML `<base>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base#attributes)
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

impl<R: BaseRecipe> HasBaseAttrs for HtmlBase<R> {
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
/// assert_eq!(base.bake(), r#"<base id="document_base_url" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let base = base!(@href "https://www.example.com");
///
/// assert_eq!(base.bake(), r#"<base href="https://www.example.com" />"#);
/// ```
#[macro_export]
macro_rules! base {
    () => {
        $crate::html::HtmlBase::new()
    };

    (@href $href:expr $(,)?) => {
        $crate::html::HtmlBase::from_href($href)
    };
    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlBase::<$r>::from_cookbook()
    };
}
