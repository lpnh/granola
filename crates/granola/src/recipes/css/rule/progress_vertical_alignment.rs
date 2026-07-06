use crate::{prelude::*, recipes::*};

/// The `progress { vertical-align: baseline }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(ProgressVerticalAlignment);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "progress {
///   vertical-align: baseline;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProgressVerticalAlignment;

impl RuleRecipe for ProgressVerticalAlignment {
    fn selectors_list_recipe() -> Bake {
        "progress".into()
    }

    fn declarations_block_recipe() -> Bake {
        CssDeclaration::from(VerticalAlign)
            .content("baseline")
            .into()
    }
}
