use askama::Template;
use std::marker::PhantomData;

use crate::prelude::*;

/// The CSS ruleset.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Syntax/Introduction#css_rulesets)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_rule: CssRule = CssRule::new("p", ("color", "rebeccapurple"));
///
/// assert_eq!(css_rule.bake(),
/// "p {
///   color: rebeccapurple;
/// }");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration: CssDeclaration = CssDeclaration::new("color", "rgb(102, 51, 153)");
/// let css_properties_list: CssPropertiesList = CssPropertiesList::new().push(css_declaration);
///
/// let css_selector: CssSelector = CssSelector::new("p");
/// let css_selector_list: CssSelectorsList = CssSelectorsList::new().push(css_selector);
///
/// let css_rule: CssRule = CssRule::new(css_selector_list, css_properties_list);
///
/// assert_eq!(css_rule.bake(),
/// "p {
///   color: rgb(102, 51, 153);
/// }");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_selector: CssSelector = "p".into();
/// let css_declaration: CssDeclaration = ("color", "rebeccapurple").into();
///
/// let css_rule: CssRule = (css_selector, css_declaration).into();
///
/// assert_eq!(css_rule.bake(),
/// "p {
///   color: rebeccapurple;
/// }");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_rule: CssRule = CssRule::new(
///     ":root",
///     [
///         ("--base-100", "oklch(93% 0.076 100.4)"),
///         ("--base-200", "oklch(90% 0.086 100.4)"),
///     ],
/// );
///
/// assert_eq!(css_rule.bake(),
/// ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_selector = ":root";
/// let css_properties_list = [
///    ("--base-100", "oklch(93% 0.076 100.4)"),
///    ("--base-200", "oklch(90% 0.086 100.4)"),
/// ];
///
/// let css_rule: CssRule = (css_selector, css_properties_list).into();
///
/// assert_eq!(css_rule.bake(),
/// ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }");
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ selectors_list }} {
///   {{ properties_list | indent(2) }}
/// }
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = RuleTag)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssRule<R: RuleTag = ()> {
    _recipe: PhantomData<R>,
    pub selectors_list: CssSelectorsList,
    pub properties_list: CssPropertiesList,
}

impl<R: RuleTag> CssRule<R> {
    pub fn new(
        selectors_list: impl Into<CssSelectorsList>,
        properties_list: impl Into<CssPropertiesList>,
    ) -> Self {
        Self {
            selectors_list: selectors_list.into(),
            properties_list: properties_list.into(),
            ..Default::default()
        }
    }

    pub fn push_selector(mut self, selector: impl Into<CssSelector>) -> Self {
        self.selectors_list = self.selectors_list.push(selector.into());
        self
    }

    pub fn push_property(mut self, declaration: impl Into<CssDeclaration>) -> Self {
        self.properties_list = self.properties_list.push(declaration.into());
        self
    }
}

impl<S: Into<CssSelectorsList>, D: Into<CssPropertiesList>> From<(S, D)> for CssRule {
    fn from((css_selectors_list, css_properties_list): (S, D)) -> Self {
        Self {
            selectors_list: css_selectors_list.into(),
            properties_list: css_properties_list.into(),
            ..Default::default()
        }
    }
}

/// Shorthand for `CssRule`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_rule = rule!("p"; ("color", "rebeccapurple"));
///
/// assert_eq!(css_rule.bake(),
/// "p {
///   color: rebeccapurple;
/// }");
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_declaration: CssDeclaration = CssDeclaration::new("color", "rgb(102, 51, 153)");
/// let css_properties_list: CssPropertiesList = CssPropertiesList::new().push(css_declaration);
///
/// let css_selector: CssSelector = CssSelector::new("p");
/// let css_selector_list: CssSelectorsList = CssSelectorsList::new().push(css_selector);
///
/// let css_rule = rule!(css_selector_list; css_properties_list);
///
/// assert_eq!(css_rule.bake(),
/// "p {
///   color: rgb(102, 51, 153);
/// }");
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_selector: CssSelector = "p".into();
/// let css_declaration: CssDeclaration = ("color", "rebeccapurple").into();
///
/// let css_rule = rule!(css_selector; css_declaration);
///
/// assert_eq!(css_rule.bake(),
/// "p {
///   color: rebeccapurple;
/// }");
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_rule: CssRule = rule!(
///     ":root";
///     ("--base-100", "oklch(93% 0.076 100.4)"),
///     ("--base-200", "oklch(90% 0.086 100.4)"),
/// );
///
/// assert_eq!(css_rule.bake(),
/// ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }");
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_selector = ":root";
/// let css_properties_list = [
///    ("--base-100", "oklch(93% 0.076 100.4)"),
///    ("--base-200", "oklch(90% 0.086 100.4)"),
/// ];
///
/// let css_rule = rule!(css_selector; css_properties_list);
///
/// assert_eq!(css_rule.bake(),
/// ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }");
/// ```
#[macro_export]
macro_rules! rule {
    ($sel: expr ; $decl: expr $(,)?) => {
        $crate::css::CssRule::<()>::new($sel, $decl)
    };
    ($sel: expr ; $first_decl: expr $(, $rest_decl: expr)+ $(,)?) => {
        $crate::css::CssRule::<()>::new($sel, [$first_decl $(, $rest_decl)*])
    };
    ($first_sel: expr $(, $rest_sel: expr)+ ; $decl: expr $(,)?) => {
        $crate::css::CssRule::<()>::new([$first_sel $(, $rest_sel)*], $decl)
    };
    ($first_sel: expr $(, $rest_sel: expr)+ ; $first_decl: expr $(, $rest_decl: expr)+ $(,)?) => {
        $crate::css::CssRule::<()>::new([$first_sel $(, $rest_sel)*], [$first_decl $(, $rest_decl)*])
    };
}
