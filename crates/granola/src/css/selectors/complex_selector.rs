use askama::Template;
use std::marker::PhantomData;

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
/// let selector: CssComplexSelector = CssComplexSelector::new("form").child("input");
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
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ComplexSelectorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssComplexSelector<R: ComplexSelectorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub first: CssCompoundSelector,
    pub rest: Vec<(CssCombinator, CssCompoundSelector)>,
}

impl<R: ComplexSelectorRecipe> CssComplexSelector<R> {
    pub fn new(selector: impl Into<CssCompoundSelector>) -> Self {
        Self {
            first: selector.into(),
            ..Default::default()
        }
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
    /// let selector: CssComplexSelector = CssComplexSelector::new("form").descendant("input");
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
    /// let selector: CssComplexSelector = CssComplexSelector::new("details").child("summary");
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
    /// let selector: CssComplexSelector = CssComplexSelector::new("label").next_sibling("input");
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
    /// let selector: CssComplexSelector = CssComplexSelector::new("img").subsequent_sibling("p");
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
    /// let selector: CssComplexSelector =
    ///     CssComplexSelector::new("column-selector").column("cell-selector");
    ///
    /// assert_eq!(selector.bake(), "column-selector || cell-selector");
    /// ```
    pub fn column(mut self, other: impl Into<CssCompoundSelector>) -> Self {
        self.rest.push((CssCombinator::Column, other.into()));
        self
    }
}

impl<R, B> From<CssCompoundSelector<R>> for CssComplexSelector<B>
where
    R: CompoundSelectorRecipe,
    B: ComplexSelectorRecipe,
{
    fn from(compound_selector: CssCompoundSelector<R>) -> Self {
        Self::new(compound_selector.bake_recipe())
    }
}

impl<R, B> From<CssSimpleSelector<R>> for CssComplexSelector<B>
where
    R: SimpleSelectorRecipe,
    B: ComplexSelectorRecipe,
{
    fn from(simple_selector: CssSimpleSelector<R>) -> Self {
        Self::new(simple_selector)
    }
}
