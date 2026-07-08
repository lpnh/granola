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
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        CssRule::from(Colors).into()
    }
}

impl RuleRecipe for Colors {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ":root".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(ColorBackground),
            CssDeclaration::from(ColorSurface),
            CssDeclaration::from(ColorBorder),
            CssDeclaration::from(ColorText),
            CssDeclaration::from(ColorPrimary),
            CssDeclaration::from(ColorPrimaryText),
            CssDeclaration::from(ColorError),
            CssDeclaration::from(ColorSuccess),
        ]
    }
}

/// The homemade recipe for the `color-background` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(ColorBackground);
///
/// assert_eq!(custom_property.bake(), "--color-background");
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

impl DeclarationRecipe for ColorBackground {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "initial".into()
    }
}

impl CustomPropertyRecipe for ColorBackground {
    fn name_recipe() -> Bake {
        "color-background".into()
    }
}

impl FnVarRecipe for ColorBackground {
    fn custom_property_recipe() -> Bake {
        "color-background".into()
    }
}

/// The homemade recipe for the `color-background` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(ColorSurface);
///
/// assert_eq!(custom_property.bake(), "--color-surface");
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

impl DeclarationRecipe for ColorSurface {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "initial".into()
    }
}

impl CustomPropertyRecipe for ColorSurface {
    fn name_recipe() -> Bake {
        "color-surface".into()
    }
}

impl FnVarRecipe for ColorSurface {
    fn custom_property_recipe() -> Bake {
        "color-surface".into()
    }
}

/// The homemade recipe for the `color-border` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(ColorBorder);
///
/// assert_eq!(custom_property.bake(), "--color-border");
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

impl DeclarationRecipe for ColorBorder {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "initial".into()
    }
}

impl CustomPropertyRecipe for ColorBorder {
    fn name_recipe() -> Bake {
        "color-border".into()
    }
}

impl FnVarRecipe for ColorBorder {
    fn custom_property_recipe() -> Bake {
        "color-border".into()
    }
}

/// The homemade recipe for the `color-text` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(ColorText);
///
/// assert_eq!(custom_property.bake(), "--color-text");
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

impl DeclarationRecipe for ColorText {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "initial".into()
    }
}

impl CustomPropertyRecipe for ColorText {
    fn name_recipe() -> Bake {
        "color-text".into()
    }
}

impl FnVarRecipe for ColorText {
    fn custom_property_recipe() -> Bake {
        "color-text".into()
    }
}

/// The homemade recipe for the `color-primary` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(ColorPrimary);
///
/// assert_eq!(custom_property.bake(), "--color-primary");
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

impl DeclarationRecipe for ColorPrimary {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "initial".into()
    }
}

impl CustomPropertyRecipe for ColorPrimary {
    fn name_recipe() -> Bake {
        "color-primary".into()
    }
}

impl FnVarRecipe for ColorPrimary {
    fn custom_property_recipe() -> Bake {
        "color-primary".into()
    }
}

/// The homemade recipe for the `color-primary-text` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(ColorPrimaryText);
///
/// assert_eq!(custom_property.bake(), "--color-primary-text");
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

impl DeclarationRecipe for ColorPrimaryText {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "initial".into()
    }
}

impl CustomPropertyRecipe for ColorPrimaryText {
    fn name_recipe() -> Bake {
        "color-primary-text".into()
    }
}

impl FnVarRecipe for ColorPrimaryText {
    fn custom_property_recipe() -> Bake {
        "color-primary-text".into()
    }
}

/// The homemade recipe for the `color-error` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(ColorError);
///
/// assert_eq!(custom_property.bake(), "--color-error");
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

impl DeclarationRecipe for ColorError {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "initial".into()
    }
}

impl CustomPropertyRecipe for ColorError {
    fn name_recipe() -> Bake {
        "color-error".into()
    }
}

impl FnVarRecipe for ColorError {
    fn custom_property_recipe() -> Bake {
        "color-error".into()
    }
}

/// The homemade recipe for the `color-success` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(ColorSuccess);
///
/// assert_eq!(custom_property.bake(), "--color-success");
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

impl DeclarationRecipe for ColorSuccess {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "initial".into()
    }
}

impl CustomPropertyRecipe for ColorSuccess {
    fn name_recipe() -> Bake {
        "color-success".into()
    }
}

impl FnVarRecipe for ColorSuccess {
    fn custom_property_recipe() -> Bake {
        "color-success".into()
    }
}
