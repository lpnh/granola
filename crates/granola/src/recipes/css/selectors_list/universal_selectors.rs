use crate::{prelude::*, recipes::*};

/// The `*, ::after, ::before` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(UniversalSelectors);
///
/// assert_eq!(rule.selectors_list, "*, ::after, ::before");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalSelectors;

impl RuleRecipe for UniversalSelectors {
    fn selectors_list_recipe() -> Bake {
        bake_comma![
            CssTypeSelector::from(Universal),
            CssSimpleSelector::from(UniversalAfter),
            CssSimpleSelector::from(UniversalBefore),
        ]
    }
}
