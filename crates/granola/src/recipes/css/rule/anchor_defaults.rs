use crate::{prelude::*, recipes::*};

/// The `a:not([class])` defaults rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(AnchorDefaults);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "a:not([class]) {
///   text-decoration-skip-ink: auto;
///   color: currentcolor;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnchorDefaults;

impl RuleRecipe for AnchorDefaults {
    fn selectors_list_recipe() -> Bake {
        "a:not([class])".into()
    }

    fn declarations_block_recipe() -> Bake {
        bake_ws![
            CssDeclaration::from(TextDecorationSkipInk).content("auto"),
            CssDeclaration::from(Color).content("currentcolor"),
        ]
    }
}
