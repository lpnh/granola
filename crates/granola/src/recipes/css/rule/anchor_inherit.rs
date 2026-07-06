use crate::{prelude::*, recipes::*};

/// The `a` inherit rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(AnchorInherit);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "a {
///   color: inherit;
///   -webkit-text-decoration: inherit;
///   text-decoration: inherit;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnchorInherit;

impl RuleRecipe for AnchorInherit {
    fn selectors_list_recipe() -> Bake {
        "a".into()
    }

    fn declarations_block_recipe() -> Bake {
        bake_ws![
            CssDeclaration::from(Color).inherit(),
            CssDeclaration::from(WebkitTextDecoration).inherit(),
            CssDeclaration::from(TextDecoration).inherit(),
        ]
    }
}
