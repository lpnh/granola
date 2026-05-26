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
#[recipe(name = SelectorTag)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssSelector<R: SelectorTag = ()> {
    _recipe: PhantomData<R>,
    pub selector: Cow<'static, str>,
}

impl<R: SelectorTag> CssSelector<R> {
    pub fn new(selector: impl Into<Cow<'static, str>>) -> Self {
        Self {
            selector: selector.into(),
            ..Default::default()
        }
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
/// {%- if !loop.last %}, {% endif -%}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssSelectorsList {
    pub selectors: Vec<CssSelector>,
}

impl CssSelectorsList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, css_selector: impl Into<CssSelector>) -> Self {
        self.selectors.push(css_selector.into());
        self
    }

    pub fn new_selector(mut self, selector: impl Into<Cow<'static, str>>) -> Self {
        self.selectors.push(CssSelector::new(selector));
        self
    }
}

impl<S: Into<CssSelector>, const N: usize> From<[S; N]> for CssSelectorsList {
    fn from(items: [S; N]) -> Self {
        Self {
            selectors: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl<S: Into<CssSelector>> From<Vec<S>> for CssSelectorsList {
    fn from(items: Vec<S>) -> Self {
        Self {
            selectors: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl<R: SelectorTag> From<CssSelector<R>> for CssSelectorsList {
    fn from(css_selector: CssSelector<R>) -> Self {
        Self {
            selectors: vec![css_selector.bake_recipe()],
        }
    }
}

impl From<Cow<'static, str>> for CssSelectorsList {
    fn from(s: Cow<'static, str>) -> Self {
        Self {
            selectors: vec![s.into()],
        }
    }
}

impl From<&'static str> for CssSelectorsList {
    fn from(s: &'static str) -> Self {
        Self {
            selectors: vec![s.into()],
        }
    }
}

impl From<String> for CssSelectorsList {
    fn from(s: String) -> Self {
        Self {
            selectors: vec![s.into()],
        }
    }
}
