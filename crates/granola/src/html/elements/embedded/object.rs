use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<object>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let object: HtmlObject = HtmlObject::empty().id("external_object");
///
/// assert_eq!(object.bake(),
/// r#"<object id="external_object"></object>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let object: HtmlObject = HtmlObject::empty()
///     .mime_type("video/mp4")
///     .data("/videos/flower.mp4")
///     .width(420)
///     .height(420);
///
/// assert_eq!(object.bake(),
/// r#"<object type="video/mp4" data="/videos/flower.mp4" width="420" height="420"></object>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <object
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</object>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ObjectTag, content = Cow<'static, str>)]
pub struct HtmlObject<R: ObjectTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// application, document, img
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: ObjectAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<object>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- mime_type | bake_attr("type") -}}
/// {{- data | bake_attr("data") -}}
/// {{- width | bake_attr("width") -}}
/// {{- height | bake_attr("height") -}}
/// {{- form | bake_attr("form") -}}
/// {{- name | bake_attr("name") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ObjectAttrs {
    pub mime_type: Option<Cow<'static, str>>,
    pub data: Option<Cow<'static, str>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub form: Option<Cow<'static, str>>,
    pub name: Option<Cow<'static, str>>,
}

pub trait HasObjectAttrs: Sized {
    fn object_attrs_mut(&mut self) -> &mut ObjectAttrs;

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#data)
    fn data(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.object_attrs_mut().data = Some(value.into());
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.object_attrs_mut().form = Some(value.into());
        self
    }

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#height)
    fn height(mut self, value: u32) -> Self {
        self.object_attrs_mut().height = Some(value);
        self
    }

    /// Name of content navigable.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.object_attrs_mut().name = Some(value.into());
        self
    }

    /// Type of embedded resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#type)
    fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.object_attrs_mut().mime_type = Some(value.into());
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#width)
    fn width(mut self, value: u32) -> Self {
        self.object_attrs_mut().width = Some(value);
        self
    }
}

impl HasObjectAttrs for ObjectAttrs {
    fn object_attrs_mut(&mut self) -> &mut ObjectAttrs {
        self
    }
}

impl HasObjectAttrs for &mut ObjectAttrs {
    fn object_attrs_mut(&mut self) -> &mut ObjectAttrs {
        self
    }
}

impl<R: ObjectTag> HasObjectAttrs for HtmlObject<R> {
    fn object_attrs_mut(&mut self) -> &mut ObjectAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlObject`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let object = object!().id("external_object");
///
/// assert_eq!(object.bake(),
/// r#"<object id="external_object"></object>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let object = object!()
///     .mime_type("video/mp4")
///     .data("/videos/flower.mp4")
///     .width(420)
///     .height(420);
///
/// assert_eq!(object.bake(),
/// r#"<object type="video/mp4" data="/videos/flower.mp4" width="420" height="420"></object>"#);
/// ```
#[macro_export]
macro_rules! object {
    () => {
        $crate::html::HtmlObject::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlObject::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlObject::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlObject::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlObject::<()>::new($crate::bake_inline![$($content),+])
    };

    (@recipe $($r:ty),+) => {
        $crate::html::HtmlObject::<$crate::cookbook!($($r),+)>::from_recipe()
    };
    (@recipe $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlObject::<$crate::cookbook!($($r),+)>::new($content)
    };
    (@recipe $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlObject::<$crate::cookbook!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@recipe $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlObject::<$crate::cookbook!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@recipe $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlObject::<$crate::cookbook!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
