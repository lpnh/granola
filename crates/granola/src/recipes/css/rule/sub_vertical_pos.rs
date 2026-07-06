use granola::{prelude::*, recipes::*};

/// The `sub { bottom: -0.25em }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SubVerticalPos);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "sub {
///   bottom: -0.25em;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubVerticalPos;

impl RuleRecipe for SubVerticalPos {
    fn selectors_list_recipe() -> Bake {
        "sub".into()
    }

    fn declarations_block_recipe() -> Bake {
        CssDeclaration::from(Bottom).content("-0.25em").into()
    }
}
