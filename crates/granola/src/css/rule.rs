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
/// let css_rule = CssRule::new()
///     .selectors_list("p")
///     .declarations_block(("color", "rebeccapurple"));
///
/// assert_eq!(
///     css_rule.bake(),
///     "p {
///   color: rebeccapurple;
/// }"
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration = CssDeclaration::new("color", "rgb(102, 51, 153)");
/// let css_properties_list = CssDeclarationsBlock::new().push(css_declaration);
///
/// let css_selector = CssSelector::new("p");
/// let css_selector_list = CssSelectorsList::new().push(css_selector);
///
/// let css_rule = CssRule::new()
///     .selectors_list(css_selector_list)
///     .declarations_block(css_properties_list);
///
/// assert_eq!(
///     css_rule.bake(),
///     "p {
///   color: rgb(102, 51, 153);
/// }"
/// );
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
/// assert_eq!(
///     css_rule.bake(),
///     "p {
///   color: rebeccapurple;
/// }"
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_rule = CssRule::new().selectors_list(":root").declarations_block([
///     ("--base-100", "oklch(93% 0.076 100.4)"),
///     ("--base-200", "oklch(90% 0.086 100.4)"),
/// ]);
///
/// assert_eq!(
///     css_rule.bake(),
///     ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }"
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_selector = ":root";
/// let css_properties_list = [
///     ("--base-100", "oklch(93% 0.076 100.4)"),
///     ("--base-200", "oklch(90% 0.086 100.4)"),
/// ];
///
/// let css_rule: CssRule = (css_selector, css_properties_list).into();
///
/// assert_eq!(
///     css_rule.bake(),
///     ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ selectors_list }} {
///   {{ declarations_block | indent(2) }}
/// }
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = RuleRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssRule<R: RuleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub selectors_list: CssSelectorsList,
    pub declarations_block: CssDeclarationsBlock,
}

impl<R: RuleRecipe> CssRule<R> {
    pub fn selectors_list(mut self, selectors_list: impl Into<CssSelectorsList>) -> Self {
        self.selectors_list = selectors_list.into();
        self
    }

    pub fn declarations_block(
        mut self,
        declarations_block: impl Into<CssDeclarationsBlock>,
    ) -> Self {
        self.declarations_block = declarations_block.into();
        self
    }

    pub fn push_selector(mut self, selector: impl Into<CssSelector>) -> Self {
        self.selectors_list = self.selectors_list.push(selector.into());
        self
    }

    pub fn push_property(mut self, declaration: impl Into<CssDeclaration>) -> Self {
        self.declarations_block = self.declarations_block.push(declaration.into());
        self
    }
}

impl<S: Into<CssSelectorsList>, D: Into<CssDeclarationsBlock>> From<(S, D)> for CssRule {
    fn from((css_selectors_list, css_properties_list): (S, D)) -> Self {
        Self {
            selectors_list: css_selectors_list.into(),
            declarations_block: css_properties_list.into(),
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
/// let css_rule = rule!("p", ("color", "rebeccapurple"));
///
/// assert_eq!(
///     css_rule.bake(),
///     "p {
///   color: rebeccapurple;
/// }"
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_declaration = CssDeclaration::new("color", "rgb(102, 51, 153)");
/// let css_properties_list = CssDeclarationsBlock::new().push(css_declaration);
///
/// let css_selector = CssSelector::new("p");
/// let css_selector_list = CssSelectorsList::new().push(css_selector);
///
/// let css_rule = rule!(css_selector_list, css_properties_list);
///
/// assert_eq!(
///     css_rule.bake(),
///     "p {
///   color: rgb(102, 51, 153);
/// }"
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_selector: CssSelector = "p".into();
/// let css_declaration: CssDeclaration = ("color", "rebeccapurple").into();
///
/// let css_rule = rule!(css_selector, css_declaration);
///
/// assert_eq!(
///     css_rule.bake(),
///     "p {
///   color: rebeccapurple;
/// }"
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_rule = rule!(
///     @selectors ":root";
///     @declarations
///     ("--base-100", "oklch(93% 0.076 100.4)"),
///     ("--base-200", "oklch(90% 0.086 100.4)"),
/// );
///
/// assert_eq!(
///     css_rule.bake(),
///     ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }"
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_selector = ":root";
/// let css_properties_list = [
///     ("--base-100", "oklch(93% 0.076 100.4)"),
///     ("--base-200", "oklch(90% 0.086 100.4)"),
/// ];
///
/// let css_rule = rule!(css_selector, css_properties_list);
///
/// assert_eq!(
///     css_rule.bake(),
///     ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }"
/// );
/// ```
///
/// ```rust
/// use granola::{recipes::*, macros::*, prelude::*};
///
/// let rule = rule!(@cookbook BoxSizingReset);
///
/// assert_eq!(
///     rule.bake(),
///     "*,
/// ::after,
/// ::before {
///   box-sizing: border-box;
/// }"
/// );
/// ```
#[macro_export]
macro_rules! rule {
    () => {
        $crate::css::CssRule::new()
    };
    ($sel:expr, $decl:expr $(,)?) => {
        $crate::css::CssRule::new().selectors_list($sel).declarations_block($decl)
    };
    (@selectors $sel:expr ; @declarations $first_decl:expr $(, $rest_decl:expr)+ $(,)?) => {
        $crate::css::CssRule::new().selectors_list($sel).declarations_block([$first_decl $(, $rest_decl)*])
    };
    (@selectors $first_sel:expr $(, $rest_sel:expr)+ ; @declarations $decl:expr $(,)?) => {
        $crate::css::CssRule::new().selectors_list([$first_sel $(, $rest_sel)*]).declarations_block($decl)
    };
    (@selectors $first_sel:expr $(, $rest_sel:expr)+ ; @declarations $first_decl:expr $(, $rest_decl:expr)+ $(,)?) => {
        $crate::css::CssRule::new().selectors_list([$first_sel $(, $rest_sel)*]).declarations_block([$first_decl $(, $rest_decl)*])
    };
    (@cookbook $($r:ty),+) => {
        $crate::css::CssRule::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
}
