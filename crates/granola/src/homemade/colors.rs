use std::borrow::Cow;

use crate::prelude::*;

/// The homemade recipe for the color custom properties.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let stylesheet = CssStylesheet::from(Colors);
///
/// assert_eq!(
///     stylesheet.bake_pretty(),
///     r#":root {
///   --color-background: initial;
///   --color-surface: initial;
///   --color-border: initial;
///   --color-text: initial;
///   --color-primary: initial;
///   --color-primary-text: initial;
///   --color-error: initial;
///   --color-success: initial;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Colors;

impl StylesheetRecipe for Colors {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([CssRule::from(Colors).into()]);
    }
}

impl RuleRecipe for Colors {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut(":root");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssCustomProperty::from(ColorBackground).into(),
            CssCustomProperty::from(ColorSurface).into(),
            CssCustomProperty::from(ColorBorder).into(),
            CssCustomProperty::from(ColorText).into(),
            CssCustomProperty::from(ColorPrimary).into(),
            CssCustomProperty::from(ColorPrimaryText).into(),
            CssCustomProperty::from(ColorError).into(),
            CssCustomProperty::from(ColorSuccess).into(),
        ]);
    }
}

/// The homemade recipe for the `color-background` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(ColorBackground);
///
/// assert_eq!(custom_property.bake(), "--color-background: initial;");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(ColorBackground);
///
/// assert_eq!(var_fn.bake(), "var(--color-background)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ColorBackground;

impl CustomPropertyRecipe for ColorBackground {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "color-background".into();
    }

    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "initial".into();
    }
}

impl FnVarRecipe for ColorBackground {
    fn custom_property_recipe(custom_property: &mut Cow<'static, str>) {
        *custom_property = "color-background".into();
    }
}

/// The homemade recipe for the `color-background` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(ColorSurface);
///
/// assert_eq!(custom_property.bake(), "--color-surface: initial;");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(ColorSurface);
///
/// assert_eq!(var_fn.bake(), "var(--color-surface)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ColorSurface;

impl CustomPropertyRecipe for ColorSurface {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "color-surface".into();
    }

    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "initial".into();
    }
}

impl FnVarRecipe for ColorSurface {
    fn custom_property_recipe(custom_property: &mut Cow<'static, str>) {
        *custom_property = "color-surface".into();
    }
}

/// The homemade recipe for the `color-border` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(ColorBorder);
///
/// assert_eq!(custom_property.bake(), "--color-border: initial;");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(ColorBorder);
///
/// assert_eq!(var_fn.bake(), "var(--color-border)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ColorBorder;

impl CustomPropertyRecipe for ColorBorder {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "color-border".into();
    }

    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "initial".into();
    }
}

impl FnVarRecipe for ColorBorder {
    fn custom_property_recipe(custom_property: &mut Cow<'static, str>) {
        *custom_property = "color-border".into();
    }
}

/// The homemade recipe for the `color-text` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(ColorText);
///
/// assert_eq!(custom_property.bake(), "--color-text: initial;");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(ColorText);
///
/// assert_eq!(var_fn.bake(), "var(--color-text)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ColorText;

impl CustomPropertyRecipe for ColorText {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "color-text".into();
    }

    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "initial".into();
    }
}

impl FnVarRecipe for ColorText {
    fn custom_property_recipe(custom_property: &mut Cow<'static, str>) {
        *custom_property = "color-text".into();
    }
}

/// The homemade recipe for the `color-primary` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(ColorPrimary);
///
/// assert_eq!(custom_property.bake(), "--color-primary: initial;");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(ColorPrimary);
///
/// assert_eq!(var_fn.bake(), "var(--color-primary)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ColorPrimary;

impl CustomPropertyRecipe for ColorPrimary {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "color-primary".into();
    }

    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "initial".into();
    }
}

impl FnVarRecipe for ColorPrimary {
    fn custom_property_recipe(custom_property: &mut Cow<'static, str>) {
        *custom_property = "color-primary".into();
    }
}

/// The homemade recipe for the `color-primary-text` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(ColorPrimaryText);
///
/// assert_eq!(custom_property.bake(), "--color-primary-text: initial;");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(ColorPrimaryText);
///
/// assert_eq!(var_fn.bake(), "var(--color-primary-text)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ColorPrimaryText;

impl CustomPropertyRecipe for ColorPrimaryText {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "color-primary-text".into();
    }

    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "initial".into();
    }
}

impl FnVarRecipe for ColorPrimaryText {
    fn custom_property_recipe(custom_property: &mut Cow<'static, str>) {
        *custom_property = "color-primary-text".into();
    }
}

/// The homemade recipe for the `color-error` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(ColorError);
///
/// assert_eq!(custom_property.bake(), "--color-error: initial;");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(ColorError);
///
/// assert_eq!(var_fn.bake(), "var(--color-error)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ColorError;

impl CustomPropertyRecipe for ColorError {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "color-error".into();
    }

    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "initial".into();
    }
}

impl FnVarRecipe for ColorError {
    fn custom_property_recipe(custom_property: &mut Cow<'static, str>) {
        *custom_property = "color-error".into();
    }
}

/// The homemade recipe for the `color-success` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(ColorSuccess);
///
/// assert_eq!(custom_property.bake(), "--color-success: initial;");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(ColorSuccess);
///
/// assert_eq!(var_fn.bake(), "var(--color-success)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ColorSuccess;

impl CustomPropertyRecipe for ColorSuccess {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "color-success".into();
    }

    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "initial".into();
    }
}

impl FnVarRecipe for ColorSuccess {
    fn custom_property_recipe(custom_property: &mut Cow<'static, str>) {
        *custom_property = "color-success".into();
    }
}
