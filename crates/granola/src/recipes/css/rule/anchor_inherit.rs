use crate::{prelude::*, recipes::*};

/// The `a` inherit rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule: CssRule<AnchorInherit> = CssRule::from_cookbook();
///
/// assert_eq!(
///     rule.bake(),
///     "a {
///   color: inherit;
///   -webkit-text-decoration: inherit;
///   text-decoration: inherit;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnchorInherit;

impl RuleRecipe for AnchorInherit {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = "a".into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            CssColor::from(Inherit).into(),
            CssWebkitTextDecoration::<Inherit>::from_cookbook().into(),
            CssTextDecoration::from(Inherit).into(),
        ];
    }
}
