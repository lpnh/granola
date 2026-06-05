use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS simple selector.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Selectors/Selector_structure#simple_selector)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let selector = CssSimpleSelector::new().selector("p");
///
/// assert_eq!(selector.bake(), "p");
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ selector }}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = SimpleSelectorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssSimpleSelector<R: SimpleSelectorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub selector: Cow<'static, str>,
}

impl<R: SimpleSelectorRecipe> CssSimpleSelector<R> {
    pub fn selector(mut self, selector: impl Into<Cow<'static, str>>) -> Self {
        self.selector = selector.into();
        self
    }

    /// Returns a [`CssCompoundSelector`] that appends a selector to the end of
    /// this [`CssSimpleSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Selectors/Selector_structure#compound_selector)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssSimpleSelector::new()
    ///     .selector("col")
    ///     .compound(".highlighted");
    ///
    /// assert_eq!(selector.bake(), "col.highlighted");
    /// ```
    pub fn compound(self, selector: impl Into<CssSimpleSelector>) -> CssCompoundSelector {
        CssCompoundSelector::from(self).push(selector)
    }

    /// Returns a [`CssCompoundSelector`] that appends a selector after a
    /// descendant combinator (single whitespace) to the end of this
    /// [`CssSimpleSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Descendant_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssSimpleSelector::new()
    ///     .selector("form")
    ///     .descendant("input");
    ///
    /// assert_eq!(selector.bake(), "form input");
    /// ```
    pub fn descendant(self, selector: impl Into<CssCompoundSelector>) -> CssComplexSelector {
        CssComplexSelector::from(self).descendant(selector)
    }

    /// Returns a [`CssCompoundSelector`] that appends a selector after a child
    /// combinator (`>`) to the end of this [`CssSimpleSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Child_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssSimpleSelector::new()
    ///     .selector("details")
    ///     .child("summary");
    ///
    /// assert_eq!(selector.bake(), "details > summary");
    /// ```
    pub fn child(self, selector: impl Into<CssCompoundSelector>) -> CssComplexSelector {
        CssComplexSelector::from(self).child(selector)
    }

    /// Returns a [`CssCompoundSelector`] that appends a selector after a
    /// next-sibling combinator (`+`) to the end of
    /// this [`CssSimpleSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Next-sibling_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssSimpleSelector::new()
    ///     .selector("label")
    ///     .next_sibling("input");
    ///
    /// assert_eq!(selector.bake(), "label + input");
    /// ```
    pub fn next_sibling(self, selector: impl Into<CssCompoundSelector>) -> CssComplexSelector {
        CssComplexSelector::from(self).next_sibling(selector)
    }

    /// Returns a [`CssCompoundSelector`] that appends a selector after a
    /// subsequent-sibling combinator (`~`) to the end of this
    /// [`CssSimpleSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Subsequent-sibling_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssSimpleSelector::new()
    ///     .selector("input")
    ///     .subsequent_sibling("label");
    ///
    /// assert_eq!(selector.bake(), "input ~ label");
    /// ```
    pub fn subsequent_sibling(
        self,
        selector: impl Into<CssCompoundSelector>,
    ) -> CssComplexSelector {
        CssComplexSelector::from(self).subsequent_sibling(selector)
    }

    /// Returns a [`CssCompoundSelector`] that appends a selector after a column
    /// combinator (`||`) to the end of this [`CssSimpleSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Column_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssSimpleSelector::new().selector("col").column("td");
    ///
    /// assert_eq!(selector.bake(), "col || td");
    /// ```
    pub fn column(self, selector: impl Into<CssCompoundSelector>) -> CssComplexSelector {
        CssComplexSelector::from(self).column(selector)
    }
}

impl<R: TypeSelectorRecipe> From<CssTypeSelector<R>> for CssSimpleSelector {
    fn from(type_selector: CssTypeSelector<R>) -> Self {
        Self::new().selector(type_selector)
    }
}

impl From<Cow<'static, str>> for CssSimpleSelector {
    fn from(s: Cow<'static, str>) -> Self {
        Self::new().selector(s)
    }
}

impl From<&'static str> for CssSimpleSelector {
    fn from(s: &'static str) -> Self {
        Self::new().selector(s)
    }
}

impl From<String> for CssSimpleSelector {
    fn from(s: String) -> Self {
        Self::new().selector(s)
    }
}

/// Shorthand for `CssSimpleSelector`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let selector = simple_selector!("p");
///
/// assert_eq!(selector.bake(), "p");
/// ```
#[macro_export]
macro_rules! simple_selector {
    () => {
        $crate::css::CssSimpleSelector::new()
    };
    ($sel:expr $(,)?) => {
        $crate::css::CssSimpleSelector::new().selector($sel)
    };
    (@cookbook $($r:ty),+) => {
        $crate::css::CssSimpleSelector::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
}
