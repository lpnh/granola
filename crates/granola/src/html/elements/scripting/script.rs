use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<script>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let script = HtmlScript::new().id("script");
///
/// assert_eq!(script.bake(), r#"<script id="script"></script>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let script = HtmlScript::new().content(bake_newline!(r#"alert("Hello, world!");"#));
///
/// assert_eq!(
///     script.bake(),
///     r#"<script>
///   alert("Hello, world!");
/// </script>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <script
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</script>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ScriptRecipe, content = Cow<'static, str>)]
pub struct HtmlScript<R: ScriptRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: ScriptAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlScript {
    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        Self::new().src(src)
    }
}

/// The HTML `<script>` element specific attributes.
///
/// [MDN Documentation]()
///
/// # Askama template
///
/// ```askama
/// {{- script_type | bake_attr("type") -}}
/// {{- src | bake_attr("src") -}}
/// {{- blocking | bake_attr("blocking") -}}
/// {{- crossorigin | bake_attr("crossorigin") -}}
/// {{- fetchpriority | bake_attr("fetchpriority") -}}
/// {{- integrity | bake_attr("integrity") -}}
/// {{- referrerpolicy | bake_attr("referrerpolicy") -}}
/// {{- async_script | bake_bool_attr("async") -}}
/// {{- defer | bake_bool_attr("defer") -}}
/// {{- nomodule | bake_bool_attr("nomodule") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ScriptAttrs {
    pub script_type: Option<Cow<'static, str>>,
    pub src: Option<Cow<'static, str>>,
    pub blocking: Option<Cow<'static, str>>,
    pub crossorigin: Option<Cow<'static, str>>,
    pub fetchpriority: Option<Cow<'static, str>>,
    pub integrity: Option<Cow<'static, str>>,
    pub referrerpolicy: Option<Cow<'static, str>>,
    pub async_script: bool,
    pub defer: bool,
    pub nomodule: bool,
}

pub trait HasScriptAttrs: Sized {
    fn script_attrs_mut(&mut self) -> &mut ScriptAttrs;

    /// Execute script when available, without blocking while fetching.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#async)
    fn async_script(mut self, value: bool) -> Self {
        self.script_attrs_mut().async_script = value;
        self
    }

    /// Whether the element is potentially render-blocking.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#blocking)
    fn blocking(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.script_attrs_mut().blocking = Some(value.into());
        self
    }

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.script_attrs_mut().crossorigin = Some(value.into());
        self
    }

    /// Defer script execution.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#defer)
    fn defer(mut self, value: bool) -> Self {
        self.script_attrs_mut().defer = value;
        self
    }

    /// Sets the priority for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/fetchpriority)
    fn fetchpriority(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.script_attrs_mut().fetchpriority = Some(value.into());
        self
    }

    /// Integrity metadata used in Subresource Integrity checks.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#integrity)
    fn integrity(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.script_attrs_mut().integrity = Some(value.into());
        self
    }

    /// Prevents execution in user agents that support module scripts.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#nomodule)
    fn nomodule(mut self, value: bool) -> Self {
        self.script_attrs_mut().nomodule = value;
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#referrerpolicy)
    fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.script_attrs_mut().referrerpolicy = Some(value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#src)
    fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.script_attrs_mut().src = Some(value.into());
        self
    }

    /// Type of script.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script/type)
    fn script_type(mut self, value: impl Into<ScriptType>) -> Self {
        self.script_attrs_mut().script_type = Some(value.into().into());
        self
    }
}

impl HasScriptAttrs for ScriptAttrs {
    fn script_attrs_mut(&mut self) -> &mut ScriptAttrs {
        self
    }
}

impl HasScriptAttrs for &mut ScriptAttrs {
    fn script_attrs_mut(&mut self) -> &mut ScriptAttrs {
        self
    }
}

impl<R: ScriptRecipe> HasScriptAttrs for HtmlScript<R> {
    fn script_attrs_mut(&mut self) -> &mut ScriptAttrs {
        &mut self.specific_attrs
    }
}

#[derive(strum::Display, strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum ScriptType {
    /// Indicates that the script is a "classic script", containing JavaScript
    /// code. Authors are encouraged to omit the attribute if the script
    /// refers to JavaScript code rather than specify a MIME type.
    #[strum(serialize = "text/javascript")]
    JavaScriptMimeType,
    /// This value indicates that the body of the element contains an import
    /// map. The import map is a JSON object that developers can use to
    /// control how the browser resolves module specifiers when importing
    /// JavaScript modules.
    Importmap,
    /// This value causes the code to be treated as a JavaScript module. The
    /// processing of the script contents is deferred. The charset and defer
    /// attributes have no effect. Unlike classic scripts, module scripts
    /// require the use of the CORS protocol for cross-origin fetching.
    Module,
    /// This value indicates that the body of the element contains speculation
    /// rules. Speculation rules take the form of a JSON object that
    /// determine what resources should be prefetched or prerendered by the
    /// browser.
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

/// Shorthand for `HtmlScript`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let script = script!().id("script");
///
/// assert_eq!(script.bake(), r#"<script id="script"></script>"#);
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
        $crate::html::HtmlScript::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlScript::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlScript::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@src $src:expr $(,)?) => {
        $crate::html::HtmlScript::from_src($src)
    };

    (@newline $content:expr $(,)?) => {
        $crate::html::HtmlScript::new().content($crate::bake_newline!($content))
    };
    (@inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlScript::new().content($crate::bake_inline![$($content),+])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlScript::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlScript::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlScript::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlScript::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlScript::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_inline![$($content),+])
    };
}
