use granola::{prelude::*, recipes::*};

/// The `sup { top: -0.5em }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SupVerticalPos);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "sup {
///   top: -0.5em;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SupVerticalPos;

impl RuleRecipe for SupVerticalPos {
    fn selectors_list_recipe() -> Bake {
        "sup".into()
    }

    fn declarations_block_recipe() -> Bake {
        CssDeclaration::from(Top).content("-0.5em").into()
    }
}
