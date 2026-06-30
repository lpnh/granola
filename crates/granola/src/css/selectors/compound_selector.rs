use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS compound selector.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Selectors/Selector_structure#compound_selector)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let selector = CssCompoundSelector::new()
///     .type_selector("col")
///     .push(".highlighted");
///
/// assert_eq!(selector.bake(), "col.highlighted");
/// assert_eq!(selector.type_selector.unwrap().selector, "col");
/// assert_eq!(selector.selectors[0].selector, ".highlighted");
/// ```
///
/// # Askama template
///
/// ```askama
/// {%- if let Some(t) = type_selector -%}
///   {{ t }}
/// {%- endif -%}
/// {%- for s in selectors -%}
///   {{ s }}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = CompoundSelectorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssCompoundSelector<R: CompoundSelectorRecipe = ()> {
    _recipe: PhantomData<R>,
    /// The CSS type selector or universal selector value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Type_selectors)
    pub type_selector: Option<CssTypeSelector>,
    pub selectors: Vec<CssSimpleSelector>,
}

impl<R: CompoundSelectorRecipe> CssCompoundSelector<R> {
    pub fn type_selector(mut self, selector: impl Into<CssTypeSelector>) -> Self {
        self.type_selector = Some(selector.into());
        self
    }

    pub fn push(mut self, selector: impl Into<CssSimpleSelector>) -> Self {
        self.selectors.push(selector.into());
        self
    }

    /// Returns a [`CssComplexSelector`] that appends a selector after a
    /// descendant combinator (single whitespace) to the end of this
    /// [`CssCompoundSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Descendant_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssCompoundSelector::new().push(".box").descendant("h2");
    ///
    /// assert_eq!(selector.bake(), ".box h2");
    /// ```
    pub fn descendant(self, selector: impl Into<CssCompoundSelector>) -> CssComplexSelector {
        CssComplexSelector::from(self).descendant(selector)
    }

    /// Returns a [`CssComplexSelector`] that appends a selector after a child
    /// combinator (`>`) to the end of this [`CssCompoundSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Child_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssCompoundSelector::new()
    ///     .type_selector("a")
    ///     .push("#selected")
    ///     .child(".icon");
    ///
    /// assert_eq!(selector.bake(), "a#selected > .icon");
    /// ```
    pub fn child(self, selector: impl Into<CssCompoundSelector>) -> CssComplexSelector {
        CssComplexSelector::from(self).child(selector)
    }

    /// Returns a [`CssComplexSelector`] that appends a selector after a
    /// next-sibling combinator (`+`) to the end of
    /// this [`CssCompoundSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Next-sibling_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssCompoundSelector::new()
    ///     .push(".box")
    ///     .descendant("h2")
    ///     .next_sibling("p");
    ///
    /// assert_eq!(selector.bake(), ".box h2 + p");
    /// ```
    pub fn next_sibling(self, selector: impl Into<CssCompoundSelector>) -> CssComplexSelector {
        CssComplexSelector::from(self).next_sibling(selector)
    }

    /// Returns a [`CssComplexSelector`] that appends a selector after a
    /// subsequent-sibling combinator (`~`) to the end of this
    /// [`CssCompoundSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Subsequent-sibling_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssCompoundSelector::new()
    ///     .type_selector("img")
    ///     .subsequent_sibling("p");
    ///
    /// assert_eq!(selector.bake(), "img ~ p");
    /// ```
    pub fn subsequent_sibling(
        self,
        selector: impl Into<CssCompoundSelector>,
    ) -> CssComplexSelector {
        CssComplexSelector::from(self).subsequent_sibling(selector)
    }

    /// Returns a [`CssComplexSelector`] that appends a selector after a column
    /// combinator (`||`) to the end of this [`CssCompoundSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Column_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssCompoundSelector::new()
    ///     .type_selector("col")
    ///     .push(".selected")
    ///     .column("td");
    ///
    /// assert_eq!(selector.bake(), "col.selected || td");
    /// ```
    pub fn column(self, selector: impl Into<CssCompoundSelector>) -> CssComplexSelector {
        CssComplexSelector::from(self).column(selector)
    }

    /// Qualifies this [`CssCompoundSelector`]'s type selector with a namespace
    /// prefix, joined by the namespace separator (`|`).
    ///
    /// The [type selector](Self::type_selector), if [`None`], defaults to the
    /// universal selector (`*`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Namespace_separator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssCompoundSelector::new().push(".box").namespace("svg");
    ///
    /// assert_eq!(selector.bake(), "svg|*.box");
    /// ```
    pub fn namespace(mut self, namespace: impl Into<Cow<'static, str>>) -> Self {
        let type_selector = self.type_selector.take().unwrap_or_default();
        self.type_selector = Some(type_selector.namespace(namespace));
        self
    }

    /// Qualifies this [`CssCompoundSelector`]'s type selector with the
    /// any-namespace prefix (`*|`).
    ///
    /// The [type selector](Self::type_selector), if [`None`], defaults to the
    /// universal selector (`*`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Namespace_separator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssCompoundSelector::new()
    ///     .type_selector("a")
    ///     .any_namespace();
    ///
    /// assert_eq!(selector.bake(), "*|a");
    /// ```
    pub fn any_namespace(mut self) -> Self {
        let type_selector = self.type_selector.take().unwrap_or_default();
        self.type_selector = Some(type_selector.any_namespace());
        self
    }

    /// Qualifies this [`CssCompoundSelector`]'s type selector with the
    /// empty-namespace prefix (`|`).
    ///
    /// The [type selector](Self::type_selector), if [`None`], defaults to the
    /// universal selector (`*`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Namespace_separator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssCompoundSelector::new().type_selector("a").no_namespace();
    ///
    /// assert_eq!(selector.bake(), "|a");
    /// ```
    pub fn no_namespace(mut self) -> Self {
        let type_selector = self.type_selector.take().unwrap_or_default();
        self.type_selector = Some(type_selector.no_namespace());
        self
    }
}

impl<R: SimpleSelectorRecipe> From<CssSimpleSelector<R>> for CssCompoundSelector {
    fn from(simple_selector: CssSimpleSelector<R>) -> Self {
        CssCompoundSelector::new().push(simple_selector.bake_recipe())
    }
}

impl<R: TypeSelectorRecipe> From<CssTypeSelector<R>> for CssCompoundSelector {
    fn from(type_selector: CssTypeSelector<R>) -> Self {
        CssCompoundSelector::new().type_selector(type_selector.bake_recipe())
    }
}

impl From<Cow<'static, str>> for CssCompoundSelector {
    fn from(selector: Cow<'static, str>) -> Self {
        if is_type_or_universal(&selector) {
            CssCompoundSelector::new().type_selector(selector)
        } else {
            CssCompoundSelector::new().push(selector)
        }
    }
}

impl From<&'static str> for CssCompoundSelector {
    fn from(selector: &'static str) -> Self {
        Self::from(Cow::Borrowed(selector))
    }
}

impl From<String> for CssCompoundSelector {
    fn from(selector: String) -> Self {
        Self::from(Cow::Owned(selector))
    }
}

/// Shorthand for `CssCompoundSelector`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let selector = compound_selector!("col", ".highlighted");
///
/// assert_eq!(selector.bake(), "col.highlighted");
/// assert_eq!(selector.type_selector.unwrap().selector, "col");
/// assert_eq!(selector.selectors[0].selector, ".highlighted");
/// ```
#[macro_export]
macro_rules! compound_selector {
    () => {
        $crate::css::CssCompoundSelector::new()
    };
    ($sel:expr $(,)?) => {
        $crate::css::CssCompoundSelector::from($sel)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::css::CssCompoundSelector::from($first)$(.push($rest))*
    };
    (@cookbook $r:ty $(,)?) => {
        $crate::css::CssCompoundSelector::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; @push $sel:expr $(,)?) => {
        $crate::css::CssCompoundSelector::<$r>::from_cookbook().push($sel)
    };
    (@cookbook $r:ty ; @push $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::css::CssCompoundSelector::<$r>::from_cookbook().push($first)$(.push($rest))*
    };
}
