use crate::prelude::*;

/// The daisyUI `card` component.
///
/// [daisyUI Documentation](https://daisyui.com/components/card/)
///
/// # Example
///
/// ```rust
/// use granola::{
///     daisyui::{btn, card},
///     macros::*,
///     prelude::*,
/// };
///
/// let card = card!(
///     figure!(
///         img!()
///             .src("card-image.png")
///             .alt("Grass and a tall iron lattice")
///     ),
///     card_body!(
///         card_title!("Open Lawn"),
///         p!("A wide green field to lounge, read, and share quiet picnics"),
///         card_actions!(btn!("Visit")),
///     ),
/// );
///
/// assert_eq!(
///     card.bake_pretty(),
///     r#"<div class="card">
///   <figure>
///     <img src="card-image.png" alt="Grass and a tall iron lattice" />
///   </figure>
///   <div class="card-body">
///     <h2 class="card-title">Open Lawn</h2>
///     <p>A wide green field to lounge, read, and share quiet picnics</p>
///     <div class="card-actions"><button class="btn">Visit</button></div>
///   </div>
/// </div>
/// "#
/// );
/// ```
///
/// ```rust
/// use granola::{daisyui::card, prelude::*};
///
/// let card = HtmlDiv::from(card::Card)
///     .content(HtmlDiv::from(card::CardBody).content("Card text"))
///     .size(card::Size::Sm)
///     .style(card::Style::Border)
///     .modifier(card::Modifier::Side);
///
/// assert_eq!(
///     card.bake(),
///     r#"<div class="card card-sm card-border card-side"><div class="card-body">Card text</div></div>"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Card;

daisyui_component! {
    Card {
        module: "card";
        macro: "card";
        base: "card";
        parts: [
            CardBody { macro: "card_body", class: "card-body" },
            CardTitle { macro: "card_title", class: "card-title" },
            CardActions { macro: "card_actions", class: "card-actions" },
        ];
        shared: [Size];
        Style {
            Border => "card-border",
            Dash => "card-dash",
        }
        Modifier {
            Side => "card-side",
            ImageFull => "image-full",
        }
    }
}

impl DivRecipe for Card {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

impl ARecipe for Card {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

impl SectionRecipe for Card {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

impl ArticleRecipe for Card {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

/// The daisyUI `card-body` component part.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CardBody;

impl DivRecipe for CardBody {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-body")
    }
}

impl SectionRecipe for CardBody {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-body")
    }
}

impl ArticleRecipe for CardBody {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-body")
    }
}

/// The daisyUI `card-title` component part.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CardTitle;

impl H1Recipe for CardTitle {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-title")
    }
}

impl H2Recipe for CardTitle {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-title")
    }
}

impl H3Recipe for CardTitle {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-title")
    }
}

impl H4Recipe for CardTitle {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-title")
    }
}

impl DivRecipe for CardTitle {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-title")
    }
}

impl SpanRecipe for CardTitle {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-title")
    }
}

/// The daisyUI `card-actions` component part.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CardActions;

impl DivRecipe for CardActions {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("card-actions")
    }
}

/// Shorthand for `HtmlDiv<Card>`.
///
/// ```rust
/// use granola::{daisyui::card, macros::*, prelude::*};
///
/// let card = card!("Card content").size(card::Size::Sm);
///
/// assert_eq!(
///     card.bake(),
///     r#"<div class="card card-sm">Card content</div>"#
/// );
/// ```
#[macro_export]
macro_rules! card {
    () => {
        $crate::html::HtmlDiv::from($crate::daisyui::Card)
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDiv::from($crate::daisyui::Card).content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDiv::from($crate::daisyui::Card)
            .content($crate::bake![$first $(, $rest)*])
    };
}

/// Shorthand for `HtmlDiv<CardBody>`.
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = card_body!("Body content");
///
/// assert_eq!(body.bake(), r#"<div class="card-body">Body content</div>"#);
/// ```
#[macro_export]
macro_rules! card_body {
    () => {
        $crate::html::HtmlDiv::from($crate::daisyui::CardBody)
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDiv::from($crate::daisyui::CardBody).content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDiv::from($crate::daisyui::CardBody)
            .content($crate::bake![$first $(, $rest)*])
    };
}

/// Shorthand for `HtmlH2<CardTitle>`.
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let title = card_title!("Card Title");
///
/// assert_eq!(title.bake(), r#"<h2 class="card-title">Card Title</h2>"#);
/// ```
#[macro_export]
macro_rules! card_title {
    () => {
        $crate::html::HtmlH2::from($crate::daisyui::CardTitle)
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlH2::from($crate::daisyui::CardTitle).content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlH2::from($crate::daisyui::CardTitle)
            .content($crate::bake![$first $(, $rest)*])
    };
}

/// Shorthand for `HtmlDiv<CardActions>`.
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let actions = card_actions!(button!("Action"));
///
/// assert_eq!(
///     actions.bake(),
///     r#"<div class="card-actions"><button>Action</button></div>"#
/// );
/// ```
#[macro_export]
macro_rules! card_actions {
    () => {
        $crate::html::HtmlDiv::from($crate::daisyui::CardActions)
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDiv::from($crate::daisyui::CardActions).content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDiv::from($crate::daisyui::CardActions)
            .content($crate::bake![$first $(, $rest)*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_renders_correct_markup_and_classes() {
        let card = HtmlDiv::from(Card)
            .size(Size::Lg)
            .style(Style::Border)
            .modifier(Modifier::Side);
        assert_eq!(
            card.bake(),
            r#"<div class="card card-lg card-border card-side"></div>"#
        );

        let image_full_card = HtmlDiv::from(Card).modifier(Modifier::ImageFull);
        assert_eq!(
            image_full_card.bake(),
            r#"<div class="card image-full"></div>"#
        );

        let card_a = HtmlA::from(Card).size(Size::Sm).style(Style::Dash);
        assert_eq!(card_a.bake(), r#"<a class="card card-sm card-dash"></a>"#);
    }

    #[test]
    fn card_parts_render_correct_classes() {
        assert_eq!(
            HtmlDiv::from(CardBody).bake(),
            r#"<div class="card-body"></div>"#
        );
        assert_eq!(
            HtmlH2::from(CardTitle).bake(),
            r#"<h2 class="card-title"></h2>"#
        );
        assert_eq!(
            HtmlH1::from(CardTitle).bake(),
            r#"<h1 class="card-title"></h1>"#
        );
        assert_eq!(
            HtmlDiv::from(CardTitle).bake(),
            r#"<div class="card-title"></div>"#
        );
        assert_eq!(
            HtmlSpan::from(CardTitle).bake(),
            r#"<span class="card-title"></span>"#
        );
        assert_eq!(
            HtmlDiv::from(CardActions).bake(),
            r#"<div class="card-actions"></div>"#
        );
    }
}
