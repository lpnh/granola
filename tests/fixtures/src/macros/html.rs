use granola::{
    daisyui::{btn, link},
    homemade::*,
    macros::*,
    prelude::*,
};

use crate::css::Stylesheet;

const FAVICON: &str = "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' \
    viewBox='0 0 100 100'><text y='.9em' font-size='90'>☕</text></svg>";

pub fn page() -> HtmlDocument {
    let description = meta!().name("description").content(
        "A cozy café on the corner of Oak Street and Elm Avenue, pouring coffee \
        and baking sourdough since six every morning.",
    );

    let favicon = html_link!().rel("icon").href(FAVICON);
    let stylesheet = Stylesheet::OatsAndEnds.link();
    let title = title!("Oats &amp; Ends Café");

    let skip_link = HtmlA::from(btn::Btn)
        .content("Skip to content")
        .href("#main")
        .color(btn::Color::Neutral)
        .size(btn::Size::Sm)
        .class("sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 z-50");

    let main = main![
        about_article(),
        menu_section(),
        hours_section(),
        visit_section(),
        newsletter_section(),
    ]
    .id("main")
    .tabindex(-1)
    .class("container mx-auto px-4 max-w-5xl space-y-12 pb-16");

    let body = body!(skip_link, site_header(), hero(), main, site_footer())
        .class("min-h-screen font-sans");

    let root_content = HomemadeRootContent::new()
        .push_meta(description)
        .push_title(title)
        .push_link(favicon)
        .push_link(stylesheet)
        .body(body);

    HtmlDocument::new().content(
        root!(root_content)
            .lang("en")
            .class("motion-safe:scroll-smooth"),
    )
}

fn site_header() -> HtmlHeader {
    let brand = a!("Oats &amp; Ends")
        .href("/")
        .class("text-xl font-bold font-serif");
    let start = div!(brand).class("navbar-start");

    let menu_link = link!("Menu")
        .href("#menu")
        .modifier(link::Modifier::Hover)
        .class("font-medium");
    let hours_link = link!("Hours")
        .href("#hours")
        .modifier(link::Modifier::Hover)
        .class("font-medium");
    let visit_link = link!("Visit")
        .href("#visit")
        .modifier(link::Modifier::Hover)
        .class("font-medium");

    let nav = nav!(menu_link, hours_link, visit_link)
        .aria_label("Primary")
        .class("flex items-center gap-6");

    let cta = HtmlA::from(btn::Btn)
        .content("Newsletter")
        .href("#newsletter")
        .color(btn::Color::Primary)
        .size(btn::Size::Sm);

    let end = div!(nav, cta).class("navbar-end gap-6");

    let navbar = div!(start, end).class("navbar container mx-auto px-4 max-w-5xl");

    header!(navbar).class("sticky top-0 z-20 bg-base-100 border-b border-base-300")
}

fn hero() -> HtmlSection {
    let h1 = h1!("Freshly roasted, freshly baked")
        .class("text-4xl sm:text-5xl font-serif font-bold text-balance");
    let lede = p!("We open at six and pour until the last regular leaves. \
            Come for the coffee, stay for the toast.")
    .class("text-lg text-base-content/80");

    let see_menu = HtmlA::from(btn::Btn)
        .content("See the menu")
        .href("#menu")
        .color(btn::Color::Primary);
    let get_directions = HtmlA::from(btn::Btn)
        .content("Get directions")
        .href("#visit")
        .style(btn::Style::Ghost);
    let actions = div!(see_menu, get_directions).class("flex justify-center gap-4 flex-wrap mt-6");

    let content = div!(h1, lede, actions).class("hero-content flex-col");

    section!(content).class("hero py-16 container mx-auto px-4 max-w-5xl text-center")
}

fn about_article() -> HtmlArticle {
    let h2 = h2!("Our story").class("text-2xl font-serif font-semibold mb-3");
    let p = p!(
        "Oats &amp; Ends opened on Oak Street, at the corner of Elm Avenue, \
        bringing new aromas to the block. Its cozy atmosphere draws in \
        passersby looking to treat themselves to a cup or two of good, hot \
        black coffee and a slice of something fresh from the oven.",
    );

    article!(h2, p).id("about")
}

fn menu_section() -> HtmlSection {
    let h2 = h2!("On the menu")
        .id("menu-heading")
        .class("text-2xl font-serif font-semibold");
    let intro = p!("Small menu, made in-house, changed with the seasons.")
        .class("text-lg text-base-content/80");

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
                "Oat milk information",
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
                "Sourdough information",
                "Baked fresh each morning, ask what's left.",
            ),
        ],
    );

    let groups = div!(coffee, bakery).class("grid grid-cols-1 md:grid-cols-2 gap-12 mt-8");

    section!(h2, intro, groups)
        .id("menu")
        .aria_labelledby("menu-heading")
}

fn hours_section() -> HtmlSection {
    let h2 = h2!("Hours")
        .id("hours-heading")
        .class("text-2xl font-serif font-semibold mb-4");

    let thead = thead!(tr![th!("Day").scope("col"), th!("Hours").scope("col")]);
    let tbody = tbody![
        tr!(th!("Weekdays").scope("row"), td!("6:00 – 18:00")),
        tr!(th!("Weekends").scope("row"), td!("7:00 – 16:00")),
    ]
    .class("tabular-nums");
    let table = table![
        caption!("Opening hours").class("text-left font-semibold mb-2"),
        thead,
        tbody,
    ]
    .class("table");

    let wrapper = div!(table).class("overflow-x-auto max-w-md");

    let note = p!("Holidays are a coin toss. Email us before making a special trip.")
        .class("text-sm text-base-content/70 mt-3");

    section!(h2, wrapper, note)
        .id("hours")
        .aria_labelledby("hours-heading")
}

fn visit_section() -> HtmlSection {
    let h2 = h2!("Visit")
        .id("visit-heading")
        .class("text-2xl font-serif font-semibold mb-3");

    let mail = link!("hello@oatsandends.test")
        .href("mailto:hello@oatsandends.test")
        .color(link::Color::Primary);
    let address = address!("Oak Street, corner of Elm Avenue", br!(), mail,)
        .class("not-italic leading-relaxed");

    let note = p!("No reservations. If there's a free chair, it's yours.")
        .class("text-sm text-base-content/70 mt-2");

    let email_cta = HtmlA::from(btn::Btn)
        .content("Email us")
        .href("mailto:hello@oatsandends.test")
        .style(btn::Style::Ghost);
    let actions = div!(email_cta).class("mt-4");

    section!(h2, address, note, actions)
        .id("visit")
        .aria_labelledby("visit-heading")
}

fn newsletter_section() -> HtmlSection {
    let h2 = h2!("Stay in the loop")
        .id("newsletter-heading")
        .class("text-2xl font-serif font-semibold");
    let intro = p!(
        "New seasonal drinks, bread restocks, and the occasional live music \
            night, straight to your inbox.",
    )
    .class("text-lg text-base-content/80");

    let legend = legend!("Email").class("fieldset-legend font-medium");
    let input = input!()
        .input_type(InputType::Email)
        .id("email")
        .name("email")
        .autocomplete("email")
        .placeholder("you@example.com")
        .required(true)
        .class("input min-w-64");
    let fieldset = fieldset!(legend, input).class("fieldset");
    let submit = btn!("Sign me up").color(btn::Color::Primary);

    let form = form!(fieldset, submit)
        .action("/newsletter")
        .method(FormMethod::Post)
        .class("flex flex-wrap items-end gap-4 mt-6");

    let note = p!("Just bread news and the odd event, never more than twice a month.")
        .class("text-sm text-base-content/70 mt-3");

    section!(h2, intro, form, note)
        .id("newsletter")
        .aria_labelledby("newsletter-heading")
}

fn site_footer() -> HtmlFooter {
    let copyright = small!("&copy; 2026 Oats &amp; Ends Café");
    let address = address!("Oak Street, corner of Elm Avenue").class("not-italic");

    let inner = div!(copyright, address)
        .class("container mx-auto px-4 max-w-5xl flex justify-between items-center flex-wrap gap-3 text-sm");

    footer!(inner).class("footer sm:footer-horizontal border-t border-base-300 py-8 bg-base-100")
}

fn menu_group(title: &'static str, items: impl Into<Bake>) -> HtmlDiv {
    let h3 =
        h3!(title).class("text-xl font-serif font-semibold border-b border-base-300 pb-2 mb-4");
    let list = ul!(items).class("list").role("list");

    div!(h3, list)
}

fn menu_item(name: &'static str, desc: &'static str) -> HtmlLi {
    let name_el = div!(name).class("list-col-grow font-semibold");
    let desc_el = div!(desc).class("text-sm text-base-content/70 list-col-wrap w-full");

    li!(name_el, desc_el).class("list-row flex-wrap items-baseline gap-y-1")
}

fn menu_item_with_tip(
    name: &'static str,
    desc: &'static str,
    tip_aria_label: &'static str,
    tip_note: &'static str,
) -> HtmlLi {
    let tip = info_tip(tip_aria_label, tip_note);
    let name_el = div!(name, tip).class("list-col-grow font-semibold flex items-center gap-1.5");
    let desc_el = div!(desc).class("text-sm text-base-content/70 list-col-wrap w-full");

    li!(name_el, desc_el).class("list-row flex-wrap items-baseline gap-y-1")
}

fn info_tip(aria_label: &'static str, note: &'static str) -> HtmlDiv {
    let trigger = btn!("i")
        .button_type(ButtonType::Button)
        .modifier(btn::Modifier::Circle)
        .size(btn::Size::Xs)
        .style(btn::Style::Outline)
        .aria_label(aria_label);

    div!(trigger)
        .class("tooltip tooltip-top")
        .custom_data("tip", note)
}
