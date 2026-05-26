use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<data>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/data)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let data: HtmlData = HtmlData::empty().id("data");
///
/// assert_eq!(data.bake(), r#"<data id="data"></data>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let data: HtmlData = HtmlData::new("$13.37").value("1337");
///
/// assert_eq!(data.bake(), r#"<data value="1337">$13.37</data>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <data
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</data>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DataTag, content = Cow<'static, str>)]
pub struct HtmlData<R: DataTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: DataAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<data>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/data#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- value | bake_attr("value") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct DataAttrs {
    pub value: Option<Cow<'static, str>>,
}

pub trait HasDataAttrs: Sized {
    fn data_attrs_mut(&mut self) -> &mut DataAttrs;

    /// Machine-readable value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data#value)
    fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.data_attrs_mut().value = Some(value.into());
        self
    }
}

impl HasDataAttrs for DataAttrs {
    fn data_attrs_mut(&mut self) -> &mut DataAttrs {
        self
    }
}

impl HasDataAttrs for &mut DataAttrs {
    fn data_attrs_mut(&mut self) -> &mut DataAttrs {
        self
    }
}

impl<R: DataTag> HasDataAttrs for HtmlData<R> {
    fn data_attrs_mut(&mut self) -> &mut DataAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlData`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let data = data!().id("data");
///
/// assert_eq!(data.bake(), r#"<data id="data"></data>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let data = data!("$13.37").value("1337");
///
/// assert_eq!(data.bake(), r#"<data value="1337">$13.37</data>"#);
/// ```
#[macro_export]
macro_rules! data {
    () => {
        $crate::html::HtmlData::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlData::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlData::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlData::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlData::<()>::new($crate::bake_inline![$($content),+])
    };
}
