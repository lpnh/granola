pub mod traits;
pub use traits::*;

/// Metadata for a daisyUI component, used for static reflection and scanning.
///
/// # Example
///
/// ```rust
/// use granola::daisyui::btn;
///
/// assert_eq!(btn::COMPONENT.module, "btn");
/// assert_eq!(btn::COMPONENT.type_name, "Btn");
/// assert_eq!(btn::COMPONENT.base_class, "btn");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Component {
    pub module: &'static str,     // (e.g. `"btn"`, `"link"`)
    pub type_name: &'static str,  // (e.g. `"Btn"`, `"Link"`)
    pub base_class: &'static str, // (e.g. `"btn"`, `"link"`)
    pub categories: &'static [ComponentCategory],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentCategory {
    pub name: &'static str, // (e.g. `"Color"`, `"Size"`)
    pub variants: &'static [ComponentVariant],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentVariant {
    pub variant: &'static str,    // (e.g. `"Primary"`, `"Outline"`)
    pub class_name: &'static str, // (e.g. `"btn-primary"`, `"btn-outline"`)
}

impl Component {
    // Resolves a path relative to this component.
    pub fn class_for_path(&self, path: &[&str]) -> Option<&'static str> {
        match path {
            [type_name] if *type_name == self.type_name => Some(self.base_class),
            [category, variant] => {
                let cat = self.categories.iter().find(|c| c.name == *category)?;
                let var = cat.variants.iter().find(|v| v.variant == *variant)?;
                Some(var.class_name)
            }
            _ => None,
        }
    }
}

macro_rules! define_daisyui_category {
    (@capability $component:ty, Color) => {
        impl $crate::daisyui::HasColor for $component {
            type Color = Color;
        }
    };
    (@capability $component:ty, Size) => {
        impl $crate::daisyui::HasSize for $component {
            type Size = Size;
        }
    };
    (@capability $component:ty, Style) => {
        impl $crate::daisyui::HasStyle for $component {
            type Style = Style;
        }
    };
    (@capability $component:ty, Behavior) => {
        impl $crate::daisyui::HasBehavior for $component {
            type Behavior = Behavior;
        }
    };
    (@capability $component:ty, Placement) => {
        impl $crate::daisyui::HasPlacement for $component {
            type Placement = Placement;
        }
    };
    (@capability $component:ty, Direction) => {
        impl $crate::daisyui::HasDirection for $component {
            type Direction = Direction;
        }
    };
    (@capability $component:ty, Modifier) => {
        impl $crate::daisyui::HasModifier for $component {
            type Modifier = Modifier;
        }
    };
    (@capability $component:ty, $category:ident) => {
        compile_error!(concat!(
            "unknown daisyUI category `",
            stringify!($category),
            "`"
        ));
    };

    // Shared category presets
    ($component:ident, $base:literal, Color) => {
        define_daisyui_category!($component, Color, $base, {
            Neutral => "neutral",
            Primary => "primary",
            Secondary => "secondary",
            Accent => "accent",
            Info => "info",
            Success => "success",
            Warning => "warning",
            Error => "error",
        });
    };
    ($component:ident, $base:literal, Size) => {
        define_daisyui_category!($component, Size, $base, {
            Xs => "xs",
            Sm => "sm",
            Md => "md",
            Lg => "lg",
            Xl => "xl",
        });
    };
    ($component:ident, $base:literal, $category:ident) => {
        compile_error!(concat!(
            "unknown shared daisyUI category `",
            stringify!($category),
            "`"
        ));
    };

    // Component category definition
    ($component:ident, $category:ident, $base:literal, { $($variant:ident => $suffix:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $category {
            $($variant),+
        }

        impl $category {
            pub const ALL: &[Self] = &[$(Self::$variant),+];
            pub const VARIANTS: &[$crate::daisyui::ComponentVariant] = &[
                $($crate::daisyui::ComponentVariant {
                    variant: stringify!($variant),
                    class_name: concat!($base, "-", $suffix),
                }),+
            ];
            pub const CATEGORY: $crate::daisyui::ComponentCategory = $crate::daisyui::ComponentCategory {
                name: stringify!($category),
                variants: Self::VARIANTS,
            };
        }

        impl $crate::daisyui::DaisyUIClassName for $category {
            fn class_name(self) -> &'static str {
                Self::VARIANTS[self as usize].class_name
            }
        }

        define_daisyui_category!(@capability $component, $category);
    };
}

macro_rules! daisyui_component {
    (
        $component:ident {
            base: $base:literal;
            shared: [$( $shared_category:ident ),* $(,)?];
            $(
                $category:ident {
                    $($variant:ident => $suffix:literal),+ $(,)?
                }
            )*
        }
    ) => {
        pub(crate) const BASE_CLASS: &str = $base;

        $(define_daisyui_category!($component, $base, $shared_category);)*
        $(define_daisyui_category!($component, $category, $base, { $($variant => $suffix),+ });)*

        pub const COMPONENT: $crate::daisyui::Component = $crate::daisyui::Component {
            module: $base,
            type_name: stringify!($component),
            base_class: BASE_CLASS,
            categories: &[
                $($shared_category::CATEGORY,)*
                $($category::CATEGORY,)*
            ],
        };
    };
}

mod actions;
mod navigation;

pub use actions::btn;
pub use actions::btn::Btn;
pub use navigation::link;
pub use navigation::link::Link;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories_supply_capabilities_to_each_recipe_element() {
        use crate::prelude::*;

        let button = HtmlButton::from(Btn)
            .color(btn::Color::Primary)
            .size(btn::Size::Lg)
            .style(btn::Style::Outline)
            .modifier(btn::Modifier::Wide);
        assert_eq!(
            button.bake(),
            r#"<button class="btn btn-primary btn-lg btn-outline btn-wide"></button>"#
        );

        let button_link = HtmlA::from(Btn)
            .color(btn::Color::Secondary)
            .style(btn::Style::Ghost);
        assert_eq!(
            button_link.bake(),
            r#"<a class="btn btn-secondary btn-ghost"></a>"#
        );

        let link = HtmlA::from(Link)
            .color(link::Color::Primary)
            .modifier(link::Modifier::Hover);
        assert_eq!(
            link.bake(),
            r#"<a class="link link-primary link-hover"></a>"#
        );

        assert_eq!(HtmlInput::from(Btn).bake(), r#"<input class="btn" />"#);
        assert_eq!(
            HtmlDiv::from(Btn).bake(),
            r#"<div class="btn" role="button"></div>"#
        );
    }
}
