use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS complex selector.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Selectors/Selector_structure#complex_selector)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let selector = CssComplexSelector::new().first("form").child("input");
///
/// assert_eq!(selector.bake(), "form > input");
/// ```
///
/// # Askama template
///
/// ```askama
/// {{- first -}}
/// {%- for (combinator, compound) in rest -%}
/// {{ combinator }}{{ compound }}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[recipe(name = ComplexSelectorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssComplexSelector<R: ComplexSelectorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub first: CssCompoundSelector,
    pub rest: Vec<(CssCombinator, CssCompoundSelector)>,
}

impl<R: ComplexSelectorRecipe> CssComplexSelector<R> {
    pub fn first(mut self, selector: impl Into<CssCompoundSelector>) -> Self {
        self.first = selector.into();
        self
    }

    pub fn combine(
        mut self,
        combinator: impl Into<CssCombinator>,
        selector: impl Into<CssCompoundSelector>,
    ) -> Self {
        self.rest.push((combinator.into(), selector.into()));
        self
    }

    /// Appends a selector after a descendant combinator (single whitespace) to
    /// the end of this [`CssComplexSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Descendant_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssComplexSelector::new().first("form").descendant("input");
    ///
    /// assert_eq!(selector.bake(), "form input");
    /// ```
    pub fn descendant(mut self, other: impl Into<CssCompoundSelector>) -> Self {
        self.rest.push((CssCombinator::Descendant, other.into()));
        self
    }

    /// Appends a selector after a child combinator (`>`) to the end of this
    /// [`CssComplexSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Child_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssComplexSelector::new().first("details").child("summary");
    ///
    /// assert_eq!(selector.bake(), "details > summary");
    /// ```
    pub fn child(mut self, other: impl Into<CssCompoundSelector>) -> Self {
        self.rest.push((CssCombinator::Child, other.into()));
        self
    }

    /// Appends a selector after a next-sibling combinator (`+`) to the end of
    /// this [`CssComplexSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Next-sibling_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssComplexSelector::new()
    ///     .first("label")
    ///     .next_sibling("input");
    ///
    /// assert_eq!(selector.bake(), "label + input");
    /// ```
    pub fn next_sibling(mut self, other: impl Into<CssCompoundSelector>) -> Self {
        self.rest.push((CssCombinator::NextSibling, other.into()));
        self
    }

    /// Appends a selector after a subsequent-sibling combinator (`~`) to the
    /// end of this [`CssComplexSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Subsequent-sibling_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssComplexSelector::new()
    ///     .first("img")
    ///     .subsequent_sibling("p");
    ///
    /// assert_eq!(selector.bake(), "img ~ p");
    /// ```
    pub fn subsequent_sibling(mut self, other: impl Into<CssCompoundSelector>) -> Self {
        self.rest
            .push((CssCombinator::SubsequentSibling, other.into()));
        self
    }

    /// Appends a selector after a column combinator (`||`) to the end of this
    /// [`CssComplexSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Column_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector = CssComplexSelector::new()
    ///     .first("column-selector")
    ///     .column("cell-selector");
    ///
    /// assert_eq!(selector.bake(), "column-selector || cell-selector");
    /// ```
    pub fn column(mut self, other: impl Into<CssCompoundSelector>) -> Self {
        self.rest.push((CssCombinator::Column, other.into()));
        self
    }
}

impl<R: CompoundSelectorRecipe> From<CssCompoundSelector<R>> for CssComplexSelector {
    fn from(compound_selector: CssCompoundSelector<R>) -> Self {
        Self::new().first(compound_selector.bake_recipe())
    }
}

impl<R: SimpleSelectorRecipe> From<CssSimpleSelector<R>> for CssComplexSelector {
    fn from(simple_selector: CssSimpleSelector<R>) -> Self {
        Self::new().first(simple_selector)
    }
}

impl<R: TypeSelectorRecipe> From<CssTypeSelector<R>> for CssComplexSelector {
    fn from(type_selector: CssTypeSelector<R>) -> Self {
        Self::new().first(type_selector)
    }
}

impl From<Cow<'static, str>> for CssComplexSelector {
    fn from(s: Cow<'static, str>) -> Self {
        Self::new().first(s)
    }
}

impl From<&'static str> for CssComplexSelector {
    fn from(s: &'static str) -> Self {
        Self::new().first(s)
    }
}

impl From<String> for CssComplexSelector {
    fn from(s: String) -> Self {
        Self::new().first(s)
    }
}

/// Shorthand for `CssComplexSelector`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let selector = complex_selector!("form").child("input");
///
/// assert_eq!(selector.bake(), "form > input");
/// ```
#[macro_export]
macro_rules! complex_selector {
    () => {
        $crate::css::CssComplexSelector::new()
    };
    ($sel:expr $(,)?) => {
        $crate::css::CssComplexSelector::new().first($sel)
    };
}
