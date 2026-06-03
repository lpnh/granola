use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<source />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let source: HtmlSource = HtmlSource::empty().id("media_or_image_source");
///
/// assert_eq!(source.bake(), r#"<source id="media_or_image_source" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let source: HtmlSource = HtmlSource::new("/videos/flower.mp4").mime_type("video/mp4");
///
/// assert_eq!(
///     source.bake(),
///     r#"<source src="/videos/flower.mp4" type="video/mp4" />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <source
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SourceRecipe)]
pub struct HtmlSource<R: SourceRecipe = ()> {
    _recipe: PhantomData<R>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SourceAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<R: SourceRecipe> HtmlSource<R> {
    pub fn new(src: impl Into<Cow<'static, str>>) -> Self {
        let mut global_attrs = GlobalAttrs::default();
        R::global_attrs_recipe(&mut global_attrs);

        let mut specific_attrs = SourceAttrs::default().src(src);
        R::specific_attrs_recipe(&mut specific_attrs);

        let mut global_aria_attrs = GlobalAriaAttrs::default();
        R::global_aria_attrs_recipe(&mut global_aria_attrs);

        let mut custom_data_attrs = CustomDataAttrs::default();
        R::custom_data_attrs_recipe(&mut custom_data_attrs);

        let mut event_handlers = EventHandlers::default();
        R::event_handlers_recipe(&mut event_handlers);

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

/// The HTML `<source />` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- src | bake_attr("src") -}}
/// {{- srcset | bake_attr("srcset") -}}
/// {{- media | bake_attr("media") -}}
/// {{- width | bake_attr("width") -}}
/// {{- height | bake_attr("height") -}}
/// {{- sizes | bake_attr("sizes") -}}
/// {{- mime_type | bake_attr("type") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct SourceAttrs {
    pub src: Option<Cow<'static, str>>,
    pub srcset: Option<Cow<'static, str>>,
    pub media: Option<Cow<'static, str>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub sizes: Option<Cow<'static, str>>,
    pub mime_type: Option<Cow<'static, str>>,
}

pub trait HasSourceAttrs: Sized {
    fn source_attrs_mut(&mut self) -> &mut SourceAttrs;

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#height)
    fn height(mut self, value: u32) -> Self {
        self.source_attrs_mut().height = Some(value);
        self
    }

    /// Applicable media.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#media)
    fn media(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.source_attrs_mut().media = Some(value.into());
        self
    }

    /// Image sizes for different page layouts.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#sizes)
    fn sizes(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.source_attrs_mut().sizes = Some(value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#src)
    fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.source_attrs_mut().src = Some(value.into());
        self
    }

    /// Images to use in different situations, e.g., high-resolution displays,
    /// small monitors, etc.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#srcset)
    fn srcset(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.source_attrs_mut().srcset = Some(value.into());
        self
    }

    /// Type of embedded resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#type)
    fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.source_attrs_mut().mime_type = Some(value.into());
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#width)
    fn width(mut self, value: u32) -> Self {
        self.source_attrs_mut().width = Some(value);
        self
    }
}

impl HasSourceAttrs for SourceAttrs {
    fn source_attrs_mut(&mut self) -> &mut SourceAttrs {
        self
    }
}

impl HasSourceAttrs for &mut SourceAttrs {
    fn source_attrs_mut(&mut self) -> &mut SourceAttrs {
        self
    }
}

impl<R: SourceRecipe> HasSourceAttrs for HtmlSource<R> {
    fn source_attrs_mut(&mut self) -> &mut SourceAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlSource`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let source = source!().id("media_or_image_source");
///
/// assert_eq!(source.bake(), r#"<source id="media_or_image_source" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let source = source!("/videos/flower.mp4").mime_type("video/mp4");
///
/// assert_eq!(
///     source.bake(),
///     r#"<source src="/videos/flower.mp4" type="video/mp4" />"#
/// );
/// ```
#[macro_export]
macro_rules! source {
    () => {
        $crate::html::HtmlSource::<()>::empty()
    };
    ($src: expr $(,)?) => {
        $crate::html::HtmlSource::<()>::new($src)
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlObject::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $src:expr $(,)?) => {
        $crate::html::HtmlObject::<$crate::cookbook_type!($($r),+)>::new($src)
    };
}
