use askama::{FastWritable, Template};
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
///     .push_property(("color", "rebeccapurple"));
///
/// assert_eq!(css_rule.bake(), "p { color: rebeccapurple; }");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_selector = CssSimpleSelector::new().selector("p");
///
/// let css_declaration = CssDeclaration::new()
///     .property("color")
///     .content("rgb(102, 51, 153)");
///
/// let css_rule = CssRule::new()
///     .selectors_list(css_selector)
///     .declarations_block(css_declaration);
///
/// assert_eq!(css_rule.bake(), "p { color: rgb(102, 51, 153); }");
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_selector = ":root";
///
/// let css_declarations_block = declarations_block![
///     ("--base-100", "oklch(93% 0.076 100.4)"),
///     ("--base-200", "oklch(90% 0.086 100.4)"),
/// ];
///
/// let css_rule = CssRule::new()
///     .selectors_list(css_selector)
///     .declarations_block(css_declarations_block);
///
/// assert_eq!(
///     css_rule.bake_pretty(),
///     ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }
/// "
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ selectors_list }} { {{ declarations_block }} }
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[granola(format = css)]
#[recipe(name = RuleRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssRule<R: RuleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub selectors_list: Bake,
    pub declarations_block: Bake,
}

impl<R: RuleRecipe> CssRule<R> {
    pub fn selectors_list(mut self, selectors_list: impl Into<Bake>) -> Self {
        self.selectors_list = selectors_list.into();
        self
    }

    pub fn declarations_block(mut self, declarations_block: impl Into<Bake>) -> Self {
        self.declarations_block = declarations_block.into();
        self
    }

    pub fn push_selector(mut self, selector: impl FastWritable) -> Self {
        self.selectors_list.fold_in_with(", ", selector);
        self
    }

    pub fn push_property<D: DeclarationRecipe>(
        mut self,
        declaration: impl Into<CssDeclaration<D>>,
    ) -> Self {
        self.declarations_block.fold_in_ws(declaration.into());
        self
    }

    pub fn push_selectors_list(mut self, selectors_list: impl FastWritable) -> Self {
        self.selectors_list.fold_in_with(", ", selectors_list);
        self
    }
}

impl<S: Into<Bake>, D: Into<Bake>> From<(S, D)> for CssRule {
    fn from((css_selectors_list, css_declarations_block): (S, D)) -> Self {
        Self {
            selectors_list: css_selectors_list.into(),
            declarations_block: css_declarations_block.into(),
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
/// let css_rule = rule!("p", declaration!("color", "rebeccapurple"));
///
/// assert_eq!(css_rule.bake(), "p { color: rebeccapurple; }");
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_rule = rule!()
///     .selectors_list("p")
///     .push_property(("color", "rgb(102, 51, 153)"));
///
/// assert_eq!(css_rule.bake(), "p { color: rgb(102, 51, 153); }");
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_rule = rule!(
///     ":root",
///     declarations_block![
///         ("--base-100", "oklch(93% 0.076 100.4)"),
///         ("--base-200", "oklch(90% 0.086 100.4)"),
///     ]
/// );
///
/// assert_eq!(
///     css_rule.bake_pretty(),
///     ":root {
///   --base-100: oklch(93% 0.076 100.4);
///   --base-200: oklch(90% 0.086 100.4);
/// }
/// "
/// );
/// ```
#[macro_export]
macro_rules! rule {
    () => {
        $crate::css::CssRule::new()
    };
    ($sel:expr, $decl:expr $(,)?) => {
        $crate::css::CssRule::new()
            .selectors_list($sel)
            .declarations_block($decl)
    };
}
