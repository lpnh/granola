use crate::{recipes::*, prelude::*};

/// The `a` inherit rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
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
            CssColor::<Inherit>::from_cookbook().into(),
            CssWebkitTextDecoration::<Inherit>::from_cookbook().into(),
            CssTextDecoration::<Inherit>::from_cookbook().into(),
        ];
    }
}
