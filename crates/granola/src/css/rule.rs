use askama::{FastWritable, Template};
use std::marker::PhantomData;

use crate::{filters, prelude::*};

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
///     .content(css_declaration);
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
///     .content(css_declarations_block);
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
/// {{ selectors_list }} { {{ content | kirei }} }
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[granola(format = css)]
#[recipe(name = RuleRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssRule<R: RuleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub selectors_list: Bake,
    pub content: R::Content,
}

impl<R: RuleRecipe<Content = Bake>> CssRule<R> {
    pub fn selectors_list(mut self, selectors_list: impl Into<Bake>) -> Self {
        self.selectors_list = selectors_list.into();
        self
    }

    pub fn push_selector<S: SimpleSelectorRecipe>(
        mut self,
        selector: impl Into<CssSimpleSelector<S>>,
    ) -> Self {
        self.selectors_list.fold_in_with(", ", selector.into());
        self
    }

    pub fn push_property<D: DeclarationRecipe>(
        mut self,
        declaration: impl Into<CssDeclaration<D>>,
    ) -> Self {
        self.content.fold_in_ws(declaration.into());
        self
    }

    pub fn push_selectors_list(mut self, selectors_list: impl FastWritable) -> Self {
        self.selectors_list.fold_in_with(", ", selectors_list);
        self
    }
}

impl<S: Into<Bake>, D: Into<Bake>> From<(S, D)> for CssRule {
    fn from((selectors_list, declarations_block): (S, D)) -> Self {
        Self {
            selectors_list: selectors_list.into(),
            content: declarations_block.into(),
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
        $crate::css::CssRule::from(($sel, $decl))
    };
}

#[macro_export]
macro_rules! rules {
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::bake_ws![$crate::css::CssRule::from($first) $(, $crate::css::CssRule::from($rest))*]
    };
}
