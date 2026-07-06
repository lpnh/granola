use crate::{prelude::*, recipes::*};

/// The `sub, sup` defaults rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SubSupDefaults);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "sub, sup {
///   font-size: 75%;
///   line-height: 0;
///   position: relative;
///   vertical-align: baseline;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubSupDefaults;

impl RuleRecipe for SubSupDefaults {
    fn selectors_list_recipe() -> Bake {
        bake_comma!["sub", "sup"]
    }

    fn declarations_block_recipe() -> Bake {
        bake_ws![
            CssDeclaration::from(FontSize).content("75%"),
            CssDeclaration::from(LineHeight).content("0"),
            CssDeclaration::from(Position).content("relative"),
            CssDeclaration::from(VerticalAlign).content("baseline"),
        ]
    }
}
