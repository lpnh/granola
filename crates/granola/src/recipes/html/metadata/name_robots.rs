use crate::prelude::*;

/// The `name="robots"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let robots: HtmlMeta<NameRobots> = HtmlMeta::from_content("noindex, nofollow");
///
/// assert_eq!(
///     robots.bake(),
///     r#"<meta name="robots" content="noindex, nofollow" />"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NameRobots;

impl MetaRecipe for NameRobots {
    fn specific_attrs_recipe(meta_attrs: &mut MetaAttrs) {
        meta_attrs.name("robots");
    }
}
