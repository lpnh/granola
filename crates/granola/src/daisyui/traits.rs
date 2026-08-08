use crate::html::attributes::HasGlobalAttrs;

/// A daisyUI enum that can be converted into its static CSS class name.
pub trait DaisyUIClassName: Copy + 'static {
    /// Returns the static daisyUI class name for this component value.
    fn class_name(self) -> &'static str;
}

pub trait HasColor {
    type Color: DaisyUIClassName;
}

pub trait HasSize {
    type Size: DaisyUIClassName;
}

pub trait HasStyle {
    type Style: DaisyUIClassName;
}

pub trait HasBehavior {
    type Behavior: DaisyUIClassName;
}

pub trait HasPlacement {
    type Placement: DaisyUIClassName;
}

pub trait HasDirection {
    type Direction: DaisyUIClassName;
}

pub trait HasModifier {
    type Modifier: DaisyUIClassName;
}

pub trait DaisyUIStyle: HasGlobalAttrs + Sized {
    type ComponentStyle: DaisyUIClassName;

    fn style(self, value: Self::ComponentStyle) -> Self {
        self.class(value.class_name())
    }
}

pub trait DaisyUIBehavior: HasGlobalAttrs + Sized {
    type ComponentBehavior: DaisyUIClassName;

    fn behavior(self, value: Self::ComponentBehavior) -> Self {
        self.class(value.class_name())
    }
}

pub trait DaisyUIColor: HasGlobalAttrs + Sized {
    type ComponentColor: DaisyUIClassName;

    fn color(self, value: Self::ComponentColor) -> Self {
        self.class(value.class_name())
    }
}

pub trait DaisyUISize: HasGlobalAttrs + Sized {
    type ComponentSize: DaisyUIClassName;

    fn size(self, value: Self::ComponentSize) -> Self {
        self.class(value.class_name())
    }
}

pub trait DaisyUIPlacement: HasGlobalAttrs + Sized {
    type ComponentPlacement: DaisyUIClassName;

    fn placement(self, value: Self::ComponentPlacement) -> Self {
        self.class(value.class_name())
    }
}

pub trait DaisyUIDirection: HasGlobalAttrs + Sized {
    type ComponentDirection: DaisyUIClassName;

    fn direction(self, value: Self::ComponentDirection) -> Self {
        self.class(value.class_name())
    }
}

pub trait DaisyUIModifier: HasGlobalAttrs + Sized {
    type ComponentModifier: DaisyUIClassName;

    fn modifier(self, value: Self::ComponentModifier) -> Self {
        self.class(value.class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestStyle {
        Outline,
    }
    impl DaisyUIClassName for TestStyle {
        fn class_name(self) -> &'static str {
            "test-outline"
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestBehavior {
        Dropdown,
    }
    impl DaisyUIClassName for TestBehavior {
        fn class_name(self) -> &'static str {
            "test-dropdown"
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestColor {
        Primary,
    }
    impl DaisyUIClassName for TestColor {
        fn class_name(self) -> &'static str {
            "test-primary"
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestSize {
        Lg,
    }
    impl DaisyUIClassName for TestSize {
        fn class_name(self) -> &'static str {
            "test-lg"
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestPlacement {
        Top,
    }
    impl DaisyUIClassName for TestPlacement {
        fn class_name(self) -> &'static str {
            "test-top"
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestDirection {
        Horizontal,
    }
    impl DaisyUIClassName for TestDirection {
        fn class_name(self) -> &'static str {
            "test-horizontal"
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestModifier {
        Active,
    }
    impl DaisyUIClassName for TestModifier {
        fn class_name(self) -> &'static str {
            "test-active"
        }
    }

    #[derive(Default, Debug, Clone, PartialEq)]
    struct Dummy;

    impl DivRecipe for Dummy {
        recipe_boilerplate!(DivRecipe);
    }

    impl HasStyle for Dummy {
        type Style = TestStyle;
    }
    impl HasBehavior for Dummy {
        type Behavior = TestBehavior;
    }
    impl HasColor for Dummy {
        type Color = TestColor;
    }
    impl HasSize for Dummy {
        type Size = TestSize;
    }
    impl HasPlacement for Dummy {
        type Placement = TestPlacement;
    }
    impl HasDirection for Dummy {
        type Direction = TestDirection;
    }
    impl HasModifier for Dummy {
        type Modifier = TestModifier;
    }

    #[test]
    fn all_capability_traits_append_classes() {
        let el = HtmlDiv::<Dummy>::default()
            .style(TestStyle::Outline)
            .behavior(TestBehavior::Dropdown)
            .color(TestColor::Primary)
            .size(TestSize::Lg)
            .placement(TestPlacement::Top)
            .direction(TestDirection::Horizontal)
            .modifier(TestModifier::Active);

        assert_eq!(
            el.bake(),
            r#"<div class="test-outline test-dropdown test-primary test-lg test-top test-horizontal test-active"></div>"#
        );
    }

    fn apply_color<T: DaisyUIColor<ComponentColor = TestColor>>(element: T) -> T {
        element.color(TestColor::Primary)
    }

    #[test]
    fn generic_capability_bound_works() {
        let el = apply_color(HtmlDiv::<Dummy>::default());
        assert_eq!(el.bake(), r#"<div class="test-primary"></div>"#);
    }
}
