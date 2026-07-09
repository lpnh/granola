use granola::{homemade::*, macros::*, prelude::*};

const FAVICON: &str = "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' \
    viewBox='0 0 100 100'><text y='.9em' font-size='90'>☕</text></svg>";

pub fn page() -> HtmlDocument<Homemade> {
    let description = meta!().name("description").content(
        "A cozy café on the corner of Oak Street and Elm Avenue, pouring coffee \
        and baking sourdough since six every morning.",
    );
    let theme_light = meta!()
        .name("theme-color")
        .content("#fbf4e8")
        .media("(prefers-color-scheme: light)");
    let theme_dark = meta!()
        .name("theme-color")
        .content("#1c140d")
        .media("(prefers-color-scheme: dark)");
    let favicon = link!().rel("icon").href(FAVICON);
    let title = title!("Oats &amp; Ends Café");
    let style = style!(super::style());

    let skip_link = a!("Skip to content").href("#main").class("skip-link");

    let main = main![
        HtmlArticle::from(AboutArticle),
        HtmlSection::from(MenuSection),
        HtmlSection::from(HoursSection),
        HtmlSection::from(VisitSection),
        HtmlSection::from(NewsletterSection),
    ]
    .id("main")
    .tabindex(-1);

    let body = body![
        skip_link,
        HtmlHeader::from(SiteHeader),
        HtmlSection::from(Hero),
        main,
        HtmlFooter::from(SiteFooter),
    ];

    HtmlDocument::from(Homemade)
        .lang("en")
        .push_meta(description)
        .push_meta(theme_light)
        .push_meta(theme_dark)
        .push_title(title)
        .push_link(favicon)
        .push_style(style)
        .body(body)
}

#[derive(Debug, Default, Clone)]
pub struct SiteHeader;

impl HeaderRecipe for SiteHeader {
    recipe_boilerplate!(HeaderRecipe, HtmlNav);

    fn content_recipe() -> Self::Content {
        let brand = a!("Oats &amp; Ends").href("/").class("brand");
        let menu_link = a!("Menu").href("#menu");
        let hours_link = a!("Hours").href("#hours");
        let visit_link = a!("Visit").href("#visit");
        let cta = a!("Newsletter")
            .href("#newsletter")
            .class("btn btn-primary");

        let nav_links = div!(menu_link, hours_link, visit_link, cta).class("nav-links");

        nav!(brand, nav_links)
            .aria_label("Primary")
            .class("site-nav wrap")
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("site-header")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Hero;

impl SectionRecipe for Hero {
    recipe_boilerplate!(SectionRecipe);

    fn content_recipe() -> Self::Content {
        let h1 = h1!("Freshly roasted, freshly baked");
        let lede = p!("We open at six and pour until the last regular leaves. \
            Come for the coffee, stay for the toast.")
        .class("lede");

        let see_menu = a!("See the menu").href("#menu").class("btn btn-primary");
        let get_directions = a!("Get directions").href("#visit").class("btn btn-ghost");
        let actions = div!(see_menu, get_directions).class("hero-actions");

        bake![h1, lede, actions]
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("hero wrap")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AboutArticle;

impl ArticleRecipe for AboutArticle {
    recipe_boilerplate!(ArticleRecipe);

    fn content_recipe() -> Self::Content {
        let h2 = h2!("Our story");
        let p = p!(
            "Oats &amp; Ends opened on Oak Street, at the corner of Elm Avenue, bringing \
        new aromas to the block. Its cozy atmosphere draws in passersby looking to \
        treat themselves to a cup or two of good, hot black coffee and a slice of \
        something fresh from the oven.",
        );

        bake![h2, p]
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().id("about").class("section wrap")
    }
}

#[derive(Debug, Default, Clone)]
pub struct MenuSection;

impl SectionRecipe for MenuSection {
    recipe_boilerplate!(SectionRecipe);

    fn content_recipe() -> Self::Content {
        let h2 = h2!("On the menu").id("menu-heading");
        let intro = p!("Small menu, made in-house, changed with the seasons.").class("lede");

        let coffee = menu_group(
            "Coffee",
            [
                menu_item(
                    "Black coffee",
                    "Drip-brewed, roasted twenty minutes down the road.",
                ),
                menu_item(
                    "Hot chocolate",
                    "Steamed whole milk, dark cocoa, no shortcuts.",
                ),
                menu_item_with_tip(
                    "Oat milk latte",
                    "Double shot, steamed oat milk.",
                    "oat-milk-tip",
                    "Dairy-free. Works in any espresso drink, just ask.",
                ),
            ],
        );

        let bakery = menu_group(
            "From the oven",
            [
                menu_item("Rustic toast", "Thick-cut, butter and jam on the side."),
                menu_item("Morning bun", "Laminated dough, cinnamon sugar crust."),
                menu_item_with_tip(
                    "Sourdough loaf",
                    "Whole loaf, ready to take home.",
                    "sourdough-tip",
                    "Baked fresh each morning, ask what's left.",
                ),
            ],
        );

        let groups = div!(coffee, bakery).class("menu-groups");

        bake![h2, intro, groups]
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().id("menu").class("section wrap")
    }

    fn global_aria_attrs_recipe() -> GlobalAriaAttrs {
        GlobalAriaAttrs::default().aria_labelledby("menu-heading")
    }
}

#[derive(Debug, Default, Clone)]
pub struct HoursSection;

impl SectionRecipe for HoursSection {
    recipe_boilerplate!(SectionRecipe);

    fn content_recipe() -> Self::Content {
        let h2 = h2!("Hours").id("hours-heading");

        let thead = thead!(tr![th!("Day").scope("col"), th!("Hours").scope("col")]);
        let tbody = tbody![
            tr!(th!("Weekdays").scope("row"), td!("6:00 – 18:00")),
            tr!(th!("Weekends").scope("row"), td!("7:00 – 16:00")),
        ];
        let table = table![caption!("Opening hours"), thead, tbody,];

        let note =
            p!("Holidays are a coin toss. Email us before making a special trip.").class("note");

        bake![h2, table, note]
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().id("hours").class("section wrap")
    }

    fn global_aria_attrs_recipe() -> GlobalAriaAttrs {
        GlobalAriaAttrs::default().aria_labelledby("hours-heading")
    }
}

#[derive(Debug, Default, Clone)]
pub struct VisitSection;

impl SectionRecipe for VisitSection {
    recipe_boilerplate!(SectionRecipe);

    fn content_recipe() -> Self::Content {
        let h2 = h2!("Visit").id("visit-heading");

        let mail = a!("hello@oatsandends.test").href("mailto:hello@oatsandends.test");
        let address = address!("Oak Street, corner of Elm Avenue", br!(), mail,);

        let note = p!("No reservations. If there's a free chair, it's yours.").class("note");

        let email_cta = a!("Email us")
            .href("mailto:hello@oatsandends.test")
            .class("btn btn-ghost");
        let actions = div!(email_cta).class("visit-actions");

        bake![h2, address, note, actions]
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().id("visit").class("section wrap")
    }

    fn global_aria_attrs_recipe() -> GlobalAriaAttrs {
        GlobalAriaAttrs::default().aria_labelledby("visit-heading")
    }
}

#[derive(Debug, Default, Clone)]
pub struct NewsletterSection;

impl SectionRecipe for NewsletterSection {
    recipe_boilerplate!(SectionRecipe);

    fn content_recipe() -> Self::Content {
        let h2 = h2!("Stay in the loop").id("newsletter-heading");
        let intro = p!(
            "New seasonal drinks, bread restocks, and the occasional live music \
            night, straight to your inbox.",
        )
        .class("lede");

        let label = label!("Email").for_id("email");
        let input = input!()
            .input_type(InputType::Email)
            .id("email")
            .name("email")
            .autocomplete("email")
            .placeholder("you@example.com")
            .required(true);
        let field = div!(label, input).class("field");
        let submit = button!("Sign me up").class("btn btn-primary");

        let form = form!(field, submit)
            .action("/newsletter")
            .method(FormMethod::Post);

        let note =
            p!("Just bread news and the odd event, never more than twice a month.").class("note");

        bake![h2, intro, form, note]
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default()
            .id("newsletter")
            .class("section wrap")
    }

    fn global_aria_attrs_recipe() -> GlobalAriaAttrs {
        GlobalAriaAttrs::default().aria_labelledby("newsletter-heading")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SiteFooter;

impl FooterRecipe for SiteFooter {
    recipe_boilerplate!(FooterRecipe, HtmlDiv);

    fn content_recipe() -> Self::Content {
        let copyright = small!("&copy; 2026 Oats &amp; Ends Café");
        let address = address!("Oak Street, corner of Elm Avenue");

        div!(copyright, address).class("wrap footer-inner")
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("site-footer")
    }
}

fn menu_group(title: &'static str, items: impl Into<ListItems>) -> HtmlDiv {
    let h3 = h3!(title);
    let list = ul!(items).class("menu-list").role("list");

    div!(h3, list).class("menu-group")
}

fn menu_item(name: &'static str, desc: &'static str) -> HtmlLi {
    let name_el = span!(name).class("menu-item-name");
    let desc_el = p!(desc).class("menu-item-desc");

    li!(name_el, desc_el)
}

fn menu_item_with_tip(
    name: &'static str,
    desc: &'static str,
    tip_id: &'static str,
    tip_note: &'static str,
) -> HtmlLi {
    let tip = info_tip(tip_id, tip_note);
    let name_el = span!(name, " ", tip).class("menu-item-name");
    let desc_el = p!(desc).class("menu-item-desc");

    li!(name_el, desc_el)
}

fn info_tip(id: &'static str, note: &'static str) -> HtmlSpan<Tooltip> {
    let trigger = HtmlButton::from(Tip).content("i");

    HtmlSpan::from(Tooltip)
        .with_id(id, trigger)
        .text(note)
        .placement(Placement::Top)
}
