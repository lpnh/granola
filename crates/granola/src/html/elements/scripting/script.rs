use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait ScriptTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlScript<Self>) -> HtmlScript<Self> {
        element
    }
}

impl ScriptTag for () {}

/// The HTML `<script>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let script: HtmlScript = HtmlScript::empty().id("script");
///
/// assert_eq!(script.bake(),
/// r#"<script id="script"></script>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let script: HtmlScript = HtmlScript::new(bake_newline!(r#"alert("Hello, world!");"#));
///
/// assert_eq!(script.bake(),
/// r#"<script>
///   alert("Hello, world!");
/// </script>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <script
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</script>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlScript<M: ScriptTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ScriptTag> HtmlScript<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let element = Self {
            content: content.into(),
            ..Default::default()
        };

        M::recipe(element)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }

    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        Self::empty().src(src)
    }

    /// Execute script when available, without blocking while fetching.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#async)
    pub fn async_(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("async");
        }
        self
    }

    /// Whether the element is potentially render-blocking.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#blocking)
    pub fn blocking(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("blocking", value);
        self
    }

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    pub fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("crossorigin", value);
        self
    }

    /// Defer script execution.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#defer)
    pub fn defer(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("defer");
        }
        self
    }

    /// Sets the priority for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/fetchpriority)
    pub fn fetchpriority(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("fetchpriority", value);
        self
    }

    /// Integrity metadata used in Subresource Integrity checks.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#integrity)
    pub fn integrity(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("integrity", value);
        self
    }

    /// Prevents execution in user agents that support module scripts.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#nomodule)
    pub fn nomodule(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("nomodule");
        }
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#referrerpolicy)
    pub fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("referrerpolicy", value);
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#src)
    pub fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("src", value);
        self
    }

    /// Type of script.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script/type)
    pub fn script_type(mut self, value: impl Into<ScriptType>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("type", value.into());
        self
    }
}

#[derive(strum::Display, strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum ScriptType {
    /// Indicates that the script is a "classic script", containing JavaScript code. Authors are
    /// encouraged to omit the attribute if the script refers to JavaScript code rather than specify
    /// a MIME type.
    #[strum(serialize = "text/javascript")]
    JavaScriptMimeType,
    /// This value indicates that the body of the element contains an import map. The import map is
    /// a JSON object that developers can use to control how the browser resolves module specifiers
    /// when importing JavaScript modules.
    Importmap,
    /// This value causes the code to be treated as a JavaScript module. The processing of the
    /// script contents is deferred. The charset and defer attributes have no effect. Unlike classic
    /// scripts, module scripts require the use of the CORS protocol for cross-origin fetching.
    Module,
    /// This value indicates that the body of the element contains speculation rules. Speculation
    /// rules take the form of a JSON object that determine what resources should be prefetched or
    /// prerendered by the browser.
    Speculationrules,
}

impl<T: AsRef<str>> From<T> for ScriptType {
    fn from(s: T) -> Self {
        let str = s.as_ref();
        match str {
            "text/javascript" => Self::JavaScriptMimeType,
            "importmap" => Self::Importmap,
            "module" => Self::Module,
            "speculationrules" => Self::Speculationrules,
            _ => Self::JavaScriptMimeType,
        }
    }
}

impl From<ScriptType> for Cow<'static, str> {
    fn from(s: ScriptType) -> Self {
        <&'static str>::from(s).into()
    }
}

/// Shorthand for `HtmlScript<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let script = script!().id("script");
///
/// assert_eq!(script.bake(),
/// r#"<script id="script"></script>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let script = script!(@newline r#"alert("Hello, world!");"#);
///
/// assert_eq!(script.bake(),
/// r#"<script>
///   alert("Hello, world!");
/// </script>"#);
/// ```
#[macro_export]
macro_rules! script {
    () => {
        $crate::html::HtmlScript::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlScript::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlScript::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlScript::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlScript::<()>::new($crate::bake_inline![$($content),+])
    };
}
