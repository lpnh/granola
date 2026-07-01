use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS type selector.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Type_selectors)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let selector = CssTypeSelector::new().selector("a").namespace("svg");
///
/// assert_eq!(selector.bake(), "svg|a");
/// ```
///
/// # Askama template
///
/// ```askama
/// {%- if let Some(ns) = namespace -%}
///     {{ ns }}|
/// {%- endif -%}
/// {{ selector }}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TypeSelectorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTypeSelector<R: TypeSelectorRecipe = ()> {
    _recipe: PhantomData<R>,
    /// The CSS namespace prefix.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Namespace_separator)
    pub namespace: Option<Cow<'static, str>>,
    pub selector: Cow<'static, str>,
}

impl<R: TypeSelectorRecipe> CssTypeSelector<R> {
    pub fn selector(mut self, selector: impl Into<Cow<'static, str>>) -> Self {
        self.selector = selector.into();
        self
    }

    /// Qualifies this [`CssTypeSelector`] with a namespace prefix, joined by
    /// the namespace separator (`|`).
    ///
    /// The [`selector`](CssTypeSelector::selector), if empty, defaults to the
    /// universal selector (`*`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Namespace_separator)
    pub fn namespace(self, prefix: impl Into<Cow<'static, str>>) -> Self {
        self.add_namespace(prefix.into())
    }

    /// Qualifies this [`CssTypeSelector`] with the any-namespace prefix (`*|`).
    ///
    /// The [`selector`](CssTypeSelector::selector), if empty, defaults to the
    /// universal selector (`*`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Namespace_separator)
    pub fn any_namespace(self) -> Self {
        self.add_namespace("*")
    }

    /// Qualifies this [`CssTypeSelector`] with the empty-namespace prefix
    /// (`|`).
    ///
    /// The [`selector`](CssTypeSelector::selector), if empty, defaults to the
    /// universal selector (`*`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Namespace_separator)
    pub fn no_namespace(self) -> Self {
        self.add_namespace("")
    }

    fn add_namespace(mut self, namespace: impl Into<Cow<'static, str>>) -> Self {
        if self.selector.is_empty() {
            self.selector = "*".into();
        }
        self.namespace = Some(namespace.into());
        self
    }
}

impl From<Cow<'static, str>> for CssTypeSelector {
    fn from(s: Cow<'static, str>) -> Self {
        Self::new().selector(s)
    }
}

impl From<&'static str> for CssTypeSelector {
    fn from(s: &'static str) -> Self {
        Self::new().selector(s)
    }
}

impl From<String> for CssTypeSelector {
    fn from(s: String) -> Self {
        Self::new().selector(s)
    }
}

/// Shorthand for `CssTypeSelector`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let selector = type_selector!("p");
///
/// assert_eq!(selector.bake(), "p");
/// ```
#[macro_export]
macro_rules! type_selector {
    () => {
        $crate::css::CssTypeSelector::new()
    };
    ($sel:expr $(,)?) => {
        $crate::css::CssTypeSelector::new().selector($sel)
    };
}
