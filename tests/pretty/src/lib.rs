#[cfg(test)]
mod tests {
    use granola::{homemade::*, macros::*, prelude::*};

    fn page_stylesheet() -> CssStylesheet {
        CssStylesheet::new()
            .push_rule((
                "body",
                declarations_block![
                    ("margin", "0"),
                    ("font-family", "system-ui, sans-serif"),
                    ("color", "#1a1a1a"),
                ],
            ))
            .push_rule((
                ".hero",
                declarations_block![
                    ("padding", "4rem 1rem"),
                    ("text-align", "center"),
                    ("background", "linear-gradient(135deg, #f6d365, #fda085)"),
                ],
            ))
            .push_rule((
                "nav a",
                declarations_block![("margin-right", "1rem"), ("text-decoration", "none")],
            ))
            .push(
                CssAtRule::new()
                    .identifier("media")
                    .rule("(min-width: 720px)")
                    .block(
                        CssRule::new()
                            .selectors_list(".menu")
                            .push_property(("display", "grid"))
                            .push_property(("grid-template-columns", "1fr 1fr"))
                            .bake(),
                    ),
            )
    }

    fn page() -> HtmlDocument<Homemade> {
        let description = HtmlMeta::new()
            .name("description")
            .content("A cozy café on the corner of Oak Street and Elm Avenue.");
        let title = HtmlTitle::new().content("Oats &amp; Ends Café");
        let style = HtmlStyle::new().content(page_stylesheet());

        let brand = HtmlA::new().content("Oats &amp; Ends").href("/");
        let nav = HtmlNav::new()
            .content(bake![
                brand,
                HtmlA::new().content("Menu").href("/menu"),
                HtmlA::new().content("Visit").href("/visit"),
            ])
            .aria_label("Primary");
        let header = HtmlHeader::new().content(nav).class("hero");

        let h1 = HtmlH1::new().content("Freshly roasted, freshly baked");
        let lede = HtmlP::new().content(
            "We open at six and pour until the last regular leaves. \
             Come for the coffee, stay for the sourdough.",
        );
        let article = HtmlArticle::new().fold_in(h1).fold_in(lede);

        let menu = HtmlUl::new()
            .content([
                HtmlLi::new().content("Black coffee"),
                HtmlLi::new().content("Hot chocolate"),
                HtmlLi::new().content("Sourdough toast"),
            ])
            .class("menu");
        let menu_section = HtmlSection::new()
            .fold_in(HtmlH2::new().content("On the menu"))
            .fold_in(menu);

        let label = HtmlLabel::new().content("Email").for_id("email");
        let input = HtmlInput::new()
            .input_type(InputType::Email)
            .id("email")
            .name("email")
            .placeholder("you@example.com")
            .required(true);
        let form = HtmlForm::new()
            .content(bake![label, input, HtmlButton::new().content("Subscribe")])
            .action("/subscribe")
            .method(FormMethod::Post);

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
        let table = HtmlTable::new().content(bake![
            HtmlCaption::new().content("Opening hours"),
            thead,
            tbody,
        ]);

        let main = HtmlMain::new().content(bake![article, menu_section, form, table]);

        let footer =
            HtmlFooter::new().content(HtmlSmall::new().content("&copy; 2026 Oats &amp; Ends Café"));

        let body = HtmlBody::new().content(bake![header, main, footer]);

        HtmlDocument::from(Homemade)
            .lang("en")
            .push_meta(description)
            .push_title(title)
            .push_style(style)
            .body(body)
    }

    macro_rules! compare_snapshot {
        ($name:ident, $build:expr) => {
            #[test]
            fn $name() {
                let value = $build;
                insta::with_settings!({
                    omit_expression => true,
                    prepend_module_to_snapshot => false,
                }, {
                    insta::assert_snapshot!(
                        concat!(stringify!($name), "_bake"),
                        value.bake()
                    );
                    insta::assert_snapshot!(
                        concat!(stringify!($name), "_pretty"),
                        value.bake_pretty()
                    );
                });
            }
        };
    }

    compare_snapshot!(full_page, page());
    compare_snapshot!(stylesheet, page_stylesheet());
}
