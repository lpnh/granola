use crate::{prelude::*, recipes::*};

/// The `b, strong { font-weight: bolder }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(BStrongFontWeight);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "b, strong {
///   font-weight: bolder;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BStrongFontWeight;

impl RuleRecipe for BStrongFontWeight {
    fn selectors_list_recipe() -> Bake {
        let mut selectors_list = Bake::default();
        for selector in ["b", "strong"] {
            selectors_list.fold_in_with(", ", selector);
        }
        selectors_list
    }

    fn declarations_block_recipe() -> Bake {
        CssFontWeight::from(Bolder).into()
    }
}
