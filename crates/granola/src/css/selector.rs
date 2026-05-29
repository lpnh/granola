use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS selector.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_selector: CssSelector = CssSelector::new("p");
///
/// assert_eq!(css_selector.bake(), "p");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_selector: CssSelector = "p".into();
///
/// assert_eq!(css_selector.bake(), "p");
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ selector }}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = SelectorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssSelector<R: SelectorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub selector: Cow<'static, str>,
}

impl<R: SelectorRecipe> CssSelector<R> {
    pub fn new(selector: impl Into<Cow<'static, str>>) -> Self {
        Self {
            selector: selector.into(),
            ..Default::default()
        }
    }

    /// Appends a selector to the end of this [`CssSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Selectors/Selector_structure#compound_selector)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector: CssSelector = CssSelector::new("col").compound(".highlighted");
    ///
    /// assert_eq!(selector.bake(), "col.highlighted");
    /// ```
    pub fn compound(mut self, selector: impl Into<Cow<'static, str>>) -> Self {
        self.selector.to_mut().push_str(selector.into().as_ref());
        self
    }

    /// Appends a selector after a child combinator (`>`) to the end of this
    /// [`CssSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Child_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector: CssSelector = CssSelector::new("details").child("summary");
    ///
    /// assert_eq!(selector.bake(), "details > summary");
    /// ```
    pub fn child(mut self, selector: impl Into<Cow<'static, str>>) -> Self {
        let s = self.selector.to_mut();
        s.push_str(" > ");
        s.push_str(selector.into().as_ref());
        self
    }

    /// Appends a selector after a column combinator (`||`) to the end of this
    /// [`CssSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Column_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector: CssSelector = CssSelector::new("col")
    ///     .compound(".highlighted")
    ///     .column("td");
    ///
    /// assert_eq!(selector.bake(), "col.highlighted || td");
    /// ```
    pub fn column(mut self, other: impl Into<Cow<'static, str>>) -> Self {
        let s = self.selector.to_mut();
        s.push_str(" || ");
        s.push_str(other.into().as_ref());
        self
    }

    /// Appends a selector after a descendant combinator (single whitespace) to
    /// the end of this [`CssSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Descendant_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector: CssSelector = CssSelector::new("form").descendant("input");
    ///
    /// assert_eq!(selector.bake(), "form input");
    /// ```
    pub fn descendant(mut self, other: impl Into<Cow<'static, str>>) -> Self {
        let s = self.selector.to_mut();
        s.push(' ');
        s.push_str(other.into().as_ref());
        self
    }

    /// Appends a selector after a next-sibling combinator (`+`) to the end of
    /// this [`CssSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Next-sibling_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector: CssSelector = CssSelector::new("label").next_sibling("input");
    ///
    /// assert_eq!(selector.bake(), "label + input");
    /// ```
    pub fn next_sibling(mut self, other: impl Into<Cow<'static, str>>) -> Self {
        let s = self.selector.to_mut();
        s.push_str(" + ");
        s.push_str(other.into().as_ref());
        self
    }

    /// Appends a selector after a subsequent-sibling combinator (`~`) to the
    /// end of this [`CssSelector`].
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/Subsequent-sibling_combinator)
    ///
    /// # Example
    ///
    /// ```rust
    /// use granola::prelude::*;
    ///
    /// let selector: CssSelector = CssSelector::new("input")
    ///     .compound(":checked")
    ///     .subsequent_sibling("label");
    ///
    /// assert_eq!(selector.bake(), "input:checked ~ label");
    /// ```
    pub fn subsequent_sibling(mut self, other: impl Into<Cow<'static, str>>) -> Self {
        let s = self.selector.to_mut();
        s.push_str(" ~ ");
        s.push_str(other.into().as_ref());
        self
    }
}

impl From<Cow<'static, str>> for CssSelector {
    fn from(s: Cow<'static, str>) -> Self {
        Self {
            selector: s,
            ..Default::default()
        }
    }
}

impl From<&'static str> for CssSelector {
    fn from(s: &'static str) -> Self {
        Self {
            selector: s.into(),
            ..Default::default()
        }
    }
}

impl From<String> for CssSelector {
    fn from(s: String) -> Self {
        Self {
            selector: s.into(),
            ..Default::default()
        }
    }
}

/// A collection of [`CssSelector`].
///
/// The selectors-list of [`CssRule`].
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_selector: CssSelector = CssSelector::new("p");
///
/// let css_selector_list: CssSelectorsList = CssSelectorsList::new().push(css_selector);
///
/// assert_eq!(css_selector_list.bake(), "p");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_selector: CssSelector = "p".into();
///
/// let css_selector_list: CssSelectorsList = css_selector.into();
///
/// assert_eq!(css_selector_list.bake(), "p");
/// ```
///
/// # Askama template
///
/// ```askama
/// {%- for s in selectors -%}
/// {{- s -}}
/// {%- if !loop.last %},
/// {% endif -%}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = SelectorsListRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssSelectorsList<R: SelectorsListRecipe = ()> {
    _recipe: PhantomData<R>,
    pub selectors: Vec<CssSelector>,
}

impl<R: SelectorsListRecipe> CssSelectorsList<R> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, css_selector: impl Into<CssSelector>) -> Self {
        self.selectors.push(css_selector.into());
        self
    }
}

impl<S: Into<CssSelector>, const N: usize> From<[S; N]> for CssSelectorsList {
    fn from(items: [S; N]) -> Self {
        Self {
            selectors: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl<S: Into<CssSelector>> From<Vec<S>> for CssSelectorsList {
    fn from(items: Vec<S>) -> Self {
        Self {
            selectors: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl<R: SelectorRecipe> From<CssSelector<R>> for CssSelectorsList {
    fn from(css_selector: CssSelector<R>) -> Self {
        Self {
            selectors: vec![css_selector.bake_recipe()],
            ..Default::default()
        }
    }
}

impl From<Cow<'static, str>> for CssSelectorsList {
    fn from(s: Cow<'static, str>) -> Self {
        Self {
            selectors: vec![s.into()],
            ..Default::default()
        }
    }
}

impl From<&'static str> for CssSelectorsList {
    fn from(s: &'static str) -> Self {
        Self {
            selectors: vec![s.into()],
            ..Default::default()
        }
    }
}

impl From<String> for CssSelectorsList {
    fn from(s: String) -> Self {
        Self {
            selectors: vec![s.into()],
            ..Default::default()
        }
    }
}
