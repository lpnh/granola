use crate::{prelude::*, recipes::*};

/// The `hr` reset rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(HrReset);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "hr {
///   height: 0;
///   color: inherit;
///   border-top-width: 1px;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HrReset;

impl RuleRecipe for HrReset {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "hr".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(Height).content("0"),
            CssDeclaration::from(Color).inherit(),
            CssDeclaration::from(BorderTopWidth).content("1px"),
        ]
    }
}
