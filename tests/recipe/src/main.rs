fn main() {}

#[cfg(test)]
mod tests_recipe {
    use askama::Template;

    use granola::{homemade::*, prelude::*};

    #[test]
    fn homemade_content_builds_document() {
        let custom_root = HomemadeRootContent::new().body(HtmlBody::new().content("custom hello"));

        let doc = HtmlDocument::new().content(HtmlRoot::new().lang("fr").content(custom_root));

        assert_eq!(
            doc.bake_pretty(),
            r#"<!DOCTYPE html>
<html lang="fr">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>custom hello</body>
</html>
"#
        );
    }

    #[test]
    fn homemade_root_content_keeps_both_fields() {
        let root_content = HomemadeRootContent::new()
            .push_title(HtmlTitle::new().content("custom title"))
            .body(HtmlBody::new().content("keep me"));

        let root = HtmlRoot::new().content(root_content);

        assert!(root.bake().contains("<title>custom title</title>"));
        assert!(root.bake().contains("<body>keep me</body>"));
    }

    #[test]
    fn homemade_root_content_preserves_head_structure() {
        let root = HtmlRoot::new().content(
            HomemadeRootContent::new()
                .push_title(HtmlTitle::new().content("replaced"))
                .push_link(HtmlLink::new().rel("stylesheet").href("style.css"))
                .push_meta(HtmlMeta::new().name("description").content("example"))
                .push_title(HtmlTitle::new().content("kept")),
        );

        let html = root.bake();
        assert!(!html.contains("replaced"));
        assert!(html.find("description").unwrap() < html.find("<title>kept</title>").unwrap());
        assert!(html.find("<title>kept</title>").unwrap() < html.find("stylesheet").unwrap());
    }

    #[test]
    fn recipe_default_hooks() {
        #[derive(Default, Debug, Clone)]
        struct Counter;

        impl ButtonRecipe for Counter {
            fn content_recipe() -> Bake {
                let count = 1 + 2;
                format!("clicked {count} times").into()
            }

            fn specific_attrs_recipe() -> ButtonAttrs {
                ButtonAttrs::default().button_type(ButtonType::Button)
            }
        }

        let button = HtmlButton::from(Counter);
        assert_eq!(
            button.bake(),
            r#"<button type="button">clicked 3 times</button>"#
        );

        let button = HtmlButton::from(Counter).content("reset");
        assert_eq!(button.bake(), r#"<button type="button">reset</button>"#);

        let baked = HtmlButton::from(Counter).bake_recipe();
        assert_eq!(
            baked.bake(),
            r#"<button type="button">clicked 3 times</button>"#
        );
        let content: Bake = baked.content;
        assert_eq!(content, "clicked 3 times");
    }

    #[test]
    fn recipe_custom_content() {
        #[derive(Default, Debug, Clone, Template, Granola)]
        #[template(
            ext = "html",
            escape = "none",
            source = "{% for tag in tags %}<b>{{ tag }}</b>{% endfor %}"
        )]
        struct TagList {
            tags: Vec<String>,
        }

        impl TagList {
            pub fn new(tag: &str) -> Self {
                Self {
                    tags: vec![String::from(tag)],
                }
            }
        }

        #[derive(Default, Debug, Clone)]
        struct Tags;

        impl DivRecipe for Tags {
            fn content_recipe() -> Bake {
                TagList {
                    tags: vec!["foo".into(), "bar".into()],
                }
                .into()
            }
        }

        let foo_bar = HtmlDiv::from(Tags);
        assert_eq!(foo_bar.bake(), "<div><b>foo</b><b>bar</b></div>");

        let foo_bar_content: Bake = foo_bar.content;
        assert_eq!(foo_bar_content, "<b>foo</b><b>bar</b>");

        let baz = HtmlDiv::from(Tags).content(TagList::new("baz"));
        assert_eq!(baz.bake(), "<div><b>baz</b></div>");

        let baz_content: Bake = baz.content;
        assert_eq!(baz_content, "<b>baz</b>");

        let baked_recipe = HtmlDiv::from(Tags).bake_recipe();
        assert_eq!(baked_recipe.bake(), "<div><b>foo</b><b>bar</b></div>");

        let baked_content: Bake = baked_recipe.content;
        assert_eq!(baked_content, "<b>foo</b><b>bar</b>");

        let baked_baz = HtmlDiv::from(Tags)
            .content(TagList::new("baz"))
            .bake_recipe();
        assert_eq!(baked_baz.bake(), "<div><b>baz</b></div>");

        let baked_baz_content: Bake = baked_baz.content;
        assert_eq!(baked_baz_content, "<b>baz</b>");
    }

    #[test]
    fn from_recipe_and_content() {
        use granola::recipes::Background;

        #[derive(Default, Debug, Clone)]
        struct Counter;

        impl ButtonRecipe for Counter {
            fn specific_attrs_recipe() -> ButtonAttrs {
                ButtonAttrs::default().button_type(ButtonType::Button)
            }
        }

        let button = HtmlButton::from((Counter, "reset"));
        assert_eq!(button.bake(), r#"<button type="button">reset</button>"#);

        let css_declaration = CssDeclaration::from((Background, "none"));
        assert_eq!(css_declaration.bake(), "background: none;");
    }
}
