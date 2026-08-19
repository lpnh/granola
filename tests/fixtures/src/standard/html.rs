use granola::{
    daisyui::{btn, link},
    homemade::*,
    prelude::*,
};

use crate::css::Stylesheet;

const FAVICON: &str = "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' \
    viewBox='0 0 100 100'><text y='.9em' font-size='90'>☕</text></svg>";

pub fn page() -> HtmlDocument {
    let description = HtmlMeta::new().name("description").content(
        "A cozy café on the corner of Oak Street and Elm Avenue, pouring coffee \
        and baking sourdough since six every morning.",
    );

    let favicon = HtmlLink::new().rel("icon").href(FAVICON);
    let stylesheet = Stylesheet::OatsAndEnds.link();
    let title = HtmlTitle::new().content("Oats &amp; Ends Café");

    let skip_link = HtmlA::from(btn::Btn)
        .content("Skip to content")
        .href("#main")
        .color(btn::Color::Neutral)
        .size(btn::Size::Sm)
        .class("sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 z-50");

    let main = HtmlMain::new()
        .fold_in(about_article())
        .fold_in(menu_section())
        .fold_in(hours_section())
        .fold_in(visit_section())
        .fold_in(newsletter_section())
        .id("main")
        .tabindex(-1)
        .class("container mx-auto px-4 max-w-5xl space-y-12 pb-16");

    let body = HtmlBody::new()
        .fold_in(skip_link)
        .fold_in(site_header())
        .fold_in(hero())
        .fold_in(main)
        .fold_in(site_footer())
        .class("min-h-screen font-sans");

    let root_content = HomemadeRootContent::new()
        .push_meta(description)
        .push_title(title)
        .push_link(favicon)
        .push_link(stylesheet)
        .body(body);

    HtmlDocument::new().content(
        HtmlRoot::new()
            .lang("en")
            .class("motion-safe:scroll-smooth")
            .content(root_content),
    )
}

fn site_header() -> HtmlHeader {
    let brand = HtmlA::new()
        .content("Oats &amp; Ends")
        .href("/")
        .class("text-xl font-bold font-serif");
    let start = HtmlDiv::new().fold_in(brand).class("navbar-start");

    let menu_link = HtmlA::from(link::Link)
        .content("Menu")
        .href("#menu")
        .modifier(link::Modifier::Hover)
        .class("font-medium");
    let hours_link = HtmlA::from(link::Link)
        .content("Hours")
        .href("#hours")
        .modifier(link::Modifier::Hover)
        .class("font-medium");
    let visit_link = HtmlA::from(link::Link)
        .content("Visit")
        .href("#visit")
        .modifier(link::Modifier::Hover)
        .class("font-medium");

    let nav = HtmlNav::new()
        .fold_in(menu_link)
        .fold_in(hours_link)
        .fold_in(visit_link)
        .aria_label("Primary")
        .class("flex items-center gap-6");

    let cta = HtmlA::from(btn::Btn)
        .content("Newsletter")
        .href("#newsletter")
        .color(btn::Color::Primary)
        .size(btn::Size::Sm);

    let end = HtmlDiv::new()
        .fold_in(nav)
        .fold_in(cta)
        .class("navbar-end gap-6");

    let navbar = HtmlDiv::new()
        .fold_in(start)
        .fold_in(end)
        .class("navbar container mx-auto px-4 max-w-5xl");

    HtmlHeader::new()
        .content(navbar)
        .class("sticky top-0 z-20 bg-base-100 border-b border-base-300")
}

fn hero() -> HtmlSection {
    let h1 = HtmlH1::new()
        .content("Freshly roasted, freshly baked")
        .class("text-4xl sm:text-5xl font-serif font-bold text-balance");
    let lede = HtmlP::new()
        .content(
            "We open at six and pour until the last regular leaves. \
            Come for the coffee, stay for the toast.",
        )
        .class("text-lg text-base-content/80");

    let see_menu = HtmlA::from(btn::Btn)
        .content("See the menu")
        .href("#menu")
        .color(btn::Color::Primary);
    let get_directions = HtmlA::from(btn::Btn)
        .content("Get directions")
        .href("#visit")
        .style(btn::Style::Ghost);
    let actions = HtmlDiv::new()
        .fold_in(see_menu)
        .fold_in(get_directions)
        .class("flex justify-center gap-4 flex-wrap mt-6");

    let content = HtmlDiv::new()
        .fold_in(h1)
        .fold_in(lede)
        .fold_in(actions)
        .class("hero-content flex-col");

    HtmlSection::new()
        .content(content)
        .class("hero py-16 container mx-auto px-4 max-w-5xl text-center")
}

fn about_article() -> HtmlArticle {
    let h2 = HtmlH2::new()
        .content("Our story")
        .class("text-2xl font-serif font-semibold mb-3");
    let p = HtmlP::new().content(
        "Oats &amp; Ends opened on Oak Street, at the corner of Elm Avenue, \
        bringing new aromas to the block. Its cozy atmosphere draws in \
        passersby looking to treat themselves to a cup or two of good, hot \
        black coffee and a slice of something fresh from the oven.",
    );

    HtmlArticle::new().fold_in(h2).fold_in(p).id("about")
}

fn menu_section() -> HtmlSection {
    let h2 = HtmlH2::new()
        .content("On the menu")
        .id("menu-heading")
        .class("text-2xl font-serif font-semibold");
    let intro = HtmlP::new()
        .content("Small menu, made in-house, changed with the seasons.")
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

    let groups = HtmlDiv::new()
        .fold_in(coffee)
        .fold_in(bakery)
        .class("grid grid-cols-1 md:grid-cols-2 gap-12 mt-8");

    HtmlSection::new()
        .fold_in(h2)
        .fold_in(intro)
        .fold_in(groups)
        .id("menu")
        .aria_labelledby("menu-heading")
}

fn hours_section() -> HtmlSection {
    let h2 = HtmlH2::new()
        .content("Hours")
        .id("hours-heading")
        .class("text-2xl font-serif font-semibold mb-4");

    let thead = HtmlThead::new().content(
        HtmlTr::new()
            .fold_in(HtmlTh::new().content("Day").scope("col"))
            .fold_in(HtmlTh::new().content("Hours").scope("col")),
    );
    let tbody = HtmlTbody::new()
        .content([
            HtmlTr::new()
                .fold_in(HtmlTh::new().content("Weekdays").scope("row"))
                .fold_in(HtmlTd::new().content("6:00 – 18:00")),
            HtmlTr::new()
                .fold_in(HtmlTh::new().content("Weekends").scope("row"))
                .fold_in(HtmlTd::new().content("7:00 – 16:00")),
        ])
        .class("tabular-nums");
    let table = HtmlTable::new()
        .fold_in(
            HtmlCaption::new()
                .content("Opening hours")
                .class("text-left font-semibold mb-2"),
        )
        .fold_in(thead)
        .fold_in(tbody)
        .class("table");

    let wrapper = HtmlDiv::new()
        .fold_in(table)
        .class("overflow-x-auto max-w-md");

    let note = HtmlP::new()
        .content("Holidays are a coin toss. Email us before making a special trip.")
        .class("text-sm text-base-content/70 mt-3");

    HtmlSection::new()
        .fold_in(h2)
        .fold_in(wrapper)
        .fold_in(note)
        .id("hours")
        .aria_labelledby("hours-heading")
}

fn visit_section() -> HtmlSection {
    let h2 = HtmlH2::new()
        .content("Visit")
        .id("visit-heading")
        .class("text-2xl font-serif font-semibold mb-3");

    let mail = HtmlA::from(link::Link)
        .content("hello@oatsandends.test")
        .href("mailto:hello@oatsandends.test")
        .color(link::Color::Primary);
    let address = HtmlAddress::new()
        .fold_in("Oak Street, corner of Elm Avenue")
        .fold_in(HtmlBr::new())
        .fold_in(mail)
        .class("not-italic leading-relaxed");

    let note = HtmlP::new()
        .content("No reservations. If there's a free chair, it's yours.")
        .class("text-sm text-base-content/70 mt-2");

    let email_cta = HtmlA::from(btn::Btn)
        .content("Email us")
        .href("mailto:hello@oatsandends.test")
        .style(btn::Style::Ghost);
    let actions = HtmlDiv::new().content(email_cta).class("mt-4");

    HtmlSection::new()
        .fold_in(h2)
        .fold_in(address)
        .fold_in(note)
        .fold_in(actions)
        .id("visit")
        .aria_labelledby("visit-heading")
}

fn newsletter_section() -> HtmlSection {
    let h2 = HtmlH2::new()
        .content("Stay in the loop")
        .id("newsletter-heading")
        .class("text-2xl font-serif font-semibold");
    let intro = HtmlP::new()
        .content(
            "New seasonal drinks, bread restocks, and the occasional live music \
            night, straight to your inbox.",
        )
        .class("text-lg text-base-content/80");

    let legend = HtmlLegend::new()
        .content("Email")
        .class("fieldset-legend font-medium");
    let input = HtmlInput::new()
        .input_type(InputType::Email)
        .id("email")
        .name("email")
        .autocomplete("email")
        .placeholder("you@example.com")
        .required(true)
        .class("input min-w-64");
    let fieldset = HtmlFieldset::new()
        .content(bake![legend, input])
        .class("fieldset");

    let submit = HtmlButton::from(btn::Btn)
        .content("Sign me up")
        .color(btn::Color::Primary);

    let form = HtmlForm::new()
        .fold_in(fieldset)
        .fold_in(submit)
        .action("/newsletter")
        .method(FormMethod::Post)
        .class("flex flex-wrap items-end gap-4 mt-6");

    let note = HtmlP::new()
        .content("Just bread news and the odd event, never more than twice a month.")
        .class("text-sm text-base-content/70 mt-3");

    HtmlSection::new()
        .fold_in(h2)
        .fold_in(intro)
        .fold_in(form)
        .fold_in(note)
        .id("newsletter")
        .aria_labelledby("newsletter-heading")
}

fn site_footer() -> HtmlFooter {
    let copyright = HtmlSmall::new().content("&copy; 2026 Oats &amp; Ends Café");
    let address = HtmlAddress::new()
        .content("Oak Street, corner of Elm Avenue")
        .class("not-italic");

    let inner = HtmlDiv::new()
        .fold_in(copyright)
        .fold_in(address)
        .class("container mx-auto px-4 max-w-5xl flex justify-between items-center flex-wrap gap-3 text-sm");

    HtmlFooter::new()
        .content(inner)
        .class("footer sm:footer-horizontal border-t border-base-300 py-8 bg-base-100")
}

fn menu_group(title: &'static str, items: impl Into<Bake>) -> HtmlDiv {
    let h3 = HtmlH3::new()
        .content(title)
        .class("text-xl font-serif font-semibold border-b border-base-300 pb-2 mb-4");
    let list = HtmlUl::new().content(items).class("list").role("list");

    HtmlDiv::new().fold_in(h3).fold_in(list)
}

fn menu_item(name: &'static str, desc: &'static str) -> HtmlLi {
    let name_el = HtmlDiv::new()
        .content(name)
        .class("list-col-grow font-semibold");
    let desc_el = HtmlDiv::new()
        .content(desc)
        .class("text-sm text-base-content/70 list-col-wrap w-full");

    HtmlLi::new()
        .fold_in(name_el)
        .fold_in(desc_el)
        .class("list-row flex-wrap items-baseline gap-y-1")
}

fn menu_item_with_tip(
    name: &'static str,
    desc: &'static str,
    tip_aria_label: &'static str,
    tip_note: &'static str,
) -> HtmlLi {
    let tip = info_tip(tip_aria_label, tip_note);
    let name_el = HtmlDiv::new()
        .fold_in(name)
        .fold_in(tip)
        .class("list-col-grow font-semibold flex items-center gap-1.5");
    let desc_el = HtmlDiv::new()
        .content(desc)
        .class("text-sm text-base-content/70 list-col-wrap w-full");

    HtmlLi::new()
        .fold_in(name_el)
        .fold_in(desc_el)
        .class("list-row flex-wrap items-baseline gap-y-1")
}

fn info_tip(aria_label: &'static str, note: &'static str) -> HtmlDiv {
    let trigger = HtmlButton::from(btn::Btn)
        .button_type(ButtonType::Button)
        .content("i")
        .modifier(btn::Modifier::Circle)
        .size(btn::Size::Xs)
        .style(btn::Style::Outline)
        .aria_label(aria_label);

    HtmlDiv::new()
        .content(trigger)
        .class("tooltip tooltip-top")
        .custom_data("tip", note)
}
