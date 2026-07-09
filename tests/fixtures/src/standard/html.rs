use granola::{homemade::*, prelude::*};

const FAVICON: &str = "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' \
    viewBox='0 0 100 100'><text y='.9em' font-size='90'>☕</text></svg>";

pub fn page() -> HtmlDocument<Homemade> {
    let description = HtmlMeta::new().name("description").content(
        "A cozy café on the corner of Oak Street and Elm Avenue, pouring coffee \
        and baking sourdough since six every morning.",
    );
    let theme_light = HtmlMeta::new()
        .name("theme-color")
        .content("#fbf4e8")
        .media("(prefers-color-scheme: light)");
    let theme_dark = HtmlMeta::new()
        .name("theme-color")
        .content("#1c140d")
        .media("(prefers-color-scheme: dark)");
    let favicon = HtmlLink::new().rel("icon").href(FAVICON);
    let title = HtmlTitle::new().content("Oats &amp; Ends Café");
    let style = HtmlStyle::new().content(super::style());

    let skip_link = HtmlA::new()
        .content("Skip to content")
        .href("#main")
        .class("skip-link");

    let main = HtmlMain::new()
        .fold_in(about_article())
        .fold_in(menu_section())
        .fold_in(hours_section())
        .fold_in(visit_section())
        .fold_in(newsletter_section())
        .id("main")
        .tabindex(-1);

    let body = HtmlBody::new()
        .fold_in(skip_link)
        .fold_in(site_header())
        .fold_in(hero())
        .fold_in(main)
        .fold_in(site_footer());

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

fn site_header() -> HtmlHeader {
    let brand = HtmlA::new()
        .content("Oats &amp; Ends")
        .href("/")
        .class("brand");

    let menu_link = HtmlA::new().content("Menu").href("#menu");
    let hours_link = HtmlA::new().content("Hours").href("#hours");
    let visit_link = HtmlA::new().content("Visit").href("#visit");
    let cta = HtmlA::new()
        .content("Newsletter")
        .href("#newsletter")
        .class("btn btn-primary");

    let nav_links = HtmlDiv::new()
        .fold_in(menu_link)
        .fold_in(hours_link)
        .fold_in(visit_link)
        .fold_in(cta)
        .class("nav-links");

    let nav = HtmlNav::new()
        .fold_in(brand)
        .fold_in(nav_links)
        .aria_label("Primary")
        .class("site-nav wrap");

    HtmlHeader::new().content(nav).class("site-header")
}

fn hero() -> HtmlSection {
    let h1 = HtmlH1::new().content("Freshly roasted, freshly baked");
    let lede = HtmlP::new()
        .content(
            "We open at six and pour until the last regular leaves. \
            Come for the coffee, stay for the toast.",
        )
        .class("lede");

    let see_menu = HtmlA::new()
        .content("See the menu")
        .href("#menu")
        .class("btn btn-primary");
    let get_directions = HtmlA::new()
        .content("Get directions")
        .href("#visit")
        .class("btn btn-ghost");
    let actions = HtmlDiv::new()
        .fold_in(see_menu)
        .fold_in(get_directions)
        .class("hero-actions");

    HtmlSection::new()
        .fold_in(h1)
        .fold_in(lede)
        .fold_in(actions)
        .class("hero wrap")
}

fn about_article() -> HtmlArticle {
    let h2 = HtmlH2::new().content("Our story");
    let p = HtmlP::new().content(
        "Oats &amp; Ends opened on Oak Street, at the corner of Elm Avenue, bringing \
        new aromas to the block. Its cozy atmosphere draws in passersby looking to \
        treat themselves to a cup or two of good, hot black coffee and a slice of \
        something fresh from the oven.",
    );

    HtmlArticle::new()
        .fold_in(h2)
        .fold_in(p)
        .id("about")
        .class("section wrap")
}

fn menu_section() -> HtmlSection {
    let h2 = HtmlH2::new().content("On the menu").id("menu-heading");
    let intro = HtmlP::new()
        .content("Small menu, made in-house, changed with the seasons.")
        .class("lede");

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

    let groups = HtmlDiv::new()
        .fold_in(coffee)
        .fold_in(bakery)
        .class("menu-groups");

    HtmlSection::new()
        .fold_in(h2)
        .fold_in(intro)
        .fold_in(groups)
        .id("menu")
        .aria_labelledby("menu-heading")
        .class("section wrap")
}

fn hours_section() -> HtmlSection {
    let h2 = HtmlH2::new().content("Hours").id("hours-heading");

    let thead = HtmlThead::new().content(
        HtmlTr::new()
            .fold_in(HtmlTh::new().content("Day").scope("col"))
            .fold_in(HtmlTh::new().content("Hours").scope("col")),
    );
    let tbody = HtmlTbody::new().content([
        HtmlTr::new()
            .fold_in(HtmlTh::new().content("Weekdays").scope("row"))
            .fold_in(HtmlTd::new().content("6:00 – 18:00")),
        HtmlTr::new()
            .fold_in(HtmlTh::new().content("Weekends").scope("row"))
            .fold_in(HtmlTd::new().content("7:00 – 16:00")),
    ]);
    let table = HtmlTable::new()
        .fold_in(HtmlCaption::new().content("Opening hours"))
        .fold_in(thead)
        .fold_in(tbody);

    let note = HtmlP::new()
        .content("Holidays are a coin toss. Email us before making a special trip.")
        .class("note");

    HtmlSection::new()
        .fold_in(h2)
        .fold_in(table)
        .fold_in(note)
        .id("hours")
        .aria_labelledby("hours-heading")
        .class("section wrap")
}

fn visit_section() -> HtmlSection {
    let h2 = HtmlH2::new().content("Visit").id("visit-heading");

    let mail = HtmlA::new()
        .content("hello@oatsandends.test")
        .href("mailto:hello@oatsandends.test");
    let address = HtmlAddress::new()
        .fold_in("Oak Street, corner of Elm Avenue")
        .fold_in(HtmlBr::new())
        .fold_in(mail);

    let note = HtmlP::new()
        .content("No reservations. If there's a free chair, it's yours.")
        .class("note");

    let email_cta = HtmlA::new()
        .content("Email us")
        .href("mailto:hello@oatsandends.test")
        .class("btn btn-ghost");
    let actions = HtmlDiv::new().content(email_cta).class("visit-actions");

    HtmlSection::new()
        .fold_in(h2)
        .fold_in(address)
        .fold_in(note)
        .fold_in(actions)
        .id("visit")
        .aria_labelledby("visit-heading")
        .class("section wrap")
}

fn newsletter_section() -> HtmlSection {
    let h2 = HtmlH2::new()
        .content("Stay in the loop")
        .id("newsletter-heading");
    let intro = HtmlP::new()
        .content(
            "New seasonal drinks, bread restocks, and the occasional live music \
            night, straight to your inbox.",
        )
        .class("lede");

    let label = HtmlLabel::new().content("Email").for_id("email");
    let input = HtmlInput::new()
        .input_type(InputType::Email)
        .id("email")
        .name("email")
        .autocomplete("email")
        .placeholder("you@example.com")
        .required(true);
    let field = HtmlDiv::new().fold_in(label).fold_in(input).class("field");
    let submit = HtmlButton::new()
        .content("Sign me up")
        .class("btn btn-primary");

    let form = HtmlForm::new()
        .fold_in(field)
        .fold_in(submit)
        .action("/newsletter")
        .method(FormMethod::Post);

    let note = HtmlP::new()
        .content("Just bread news and the odd event, never more than twice a month.")
        .class("note");

    HtmlSection::new()
        .fold_in(h2)
        .fold_in(intro)
        .fold_in(form)
        .fold_in(note)
        .id("newsletter")
        .aria_labelledby("newsletter-heading")
        .class("section wrap")
}

fn site_footer() -> HtmlFooter {
    let copyright = HtmlSmall::new().content("&copy; 2026 Oats &amp; Ends Café");
    let address = HtmlAddress::new().content("Oak Street, corner of Elm Avenue");

    let inner = HtmlDiv::new()
        .fold_in(copyright)
        .fold_in(address)
        .class("wrap footer-inner");

    HtmlFooter::new().content(inner).class("site-footer")
}

fn menu_group(title: &'static str, items: impl Into<ListItems>) -> HtmlDiv {
    let h3 = HtmlH3::new().content(title);
    let list = HtmlUl::new().content(items).class("menu-list").role("list");

    HtmlDiv::new().fold_in(h3).fold_in(list).class("menu-group")
}

fn menu_item(name: &'static str, desc: &'static str) -> HtmlLi {
    let name_el = HtmlSpan::new().content(name).class("menu-item-name");
    let desc_el = HtmlP::new().content(desc).class("menu-item-desc");

    HtmlLi::new().fold_in(name_el).fold_in(desc_el)
}

fn menu_item_with_tip(
    name: &'static str,
    desc: &'static str,
    tip_id: &'static str,
    tip_note: &'static str,
) -> HtmlLi {
    let tip = info_tip(tip_id, tip_note);
    let name_el = HtmlSpan::new()
        .fold_in(name)
        .fold_in(" ")
        .fold_in(tip)
        .class("menu-item-name");
    let desc_el = HtmlP::new().content(desc).class("menu-item-desc");

    HtmlLi::new().fold_in(name_el).fold_in(desc_el)
}

fn info_tip(id: &'static str, note: &'static str) -> HtmlSpan<Tooltip> {
    let trigger = HtmlButton::from(Tip).content("i");

    HtmlSpan::from(Tooltip)
        .with_id(id, trigger)
        .text(note)
        .placement(Placement::Top)
}
