//! Based on Material Symbols by Google
//! Source: https://github.com/google/material-design-icons
//! Licensed under Apache License Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)

mod image;
pub use image::*;

use crate::{prelude::*, recipes::*};

/// The height="24px", viewBox="0 -960 960 960", width="24px" recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let image_icon = Svg::from(MaterialSymbols);
///
/// assert_eq!(
///     image_icon.bake(),
///     r#"<svg height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"></svg>"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MaterialSymbols;

impl SvgRecipe for MaterialSymbols {
    recipe_boilerplate!(SvgRecipe);

    fn specific_attrs_recipe(specific_attrs: &mut SvgAttrs) {
        specific_attrs.height("24px");
        specific_attrs.view_box("0 -960 960 960");
        specific_attrs.width("24px");
    }

    fn paint_attrs_recipe(paint_attrs: &mut PaintAttrs) {
        FillCurrentColor::paint_attrs_recipe(paint_attrs)
    }
}
