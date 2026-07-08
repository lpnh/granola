use crate::prelude::*;

/// The `role="list"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let list = HtmlUl::from(RoleList).content(HtmlLi::new().content("sugar"));
///
/// assert_eq!(list.bake(), r#"<ul role="list"><li>sugar</li></ul>"#);
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let list = HtmlOl::from(RoleList).content(HtmlLi::new().content("Add the sugar"));
///
/// assert_eq!(
///     list.bake(),
///     r#"<ol role="list"><li>Add the sugar</li></ol>"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RoleList;

impl UlRecipe for RoleList {
    recipe_boilerplate!(UlRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().role("list")
    }
}

impl OlRecipe for RoleList {
    recipe_boilerplate!(OlRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().role("list")
    }
}
