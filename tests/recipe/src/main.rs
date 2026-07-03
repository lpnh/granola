fn main() {}

#[cfg(test)]
mod tests_recipe {
    use askama::Template;

    use granola::{homemade::*, prelude::*};

    #[test]
    fn content_replace_keeps_caller_value() {
        let mut custom_root = HtmlRoot::from(Homemade).lang("fr");
        custom_root.body(HtmlBody::new().content("custom hello"));

        let doc = HtmlDocument::from(Homemade).content(custom_root);

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
    fn content_replace_does_not_duplicate() {
        let head = HtmlHead::from(Homemade);
        let mut content = head.content.clone();
        content
            .meta
            .push(HtmlMeta::new().content("yo").bake_recipe());

        let head = head.content(content);

        assert_eq!(head.content.meta.len(), 3);
    }

    #[test]
    fn content_replace_keeps_both_fields() {
        let root = HtmlRoot::from(Homemade);

        let mut custom_head = HtmlHead::from(Homemade);
        custom_head.content.title = Some(HtmlTitle::new().content("custom title").bake_recipe());

        let mut new_content = root.content.clone();
        new_content.head = custom_head;
        new_content.body = Some(HtmlBody::new().content("keep me").bake_recipe());

        let root = root.content(new_content);

        assert!(root.bake().contains("custom title"));
        assert!(root.bake().contains("keep me"));
    }

    #[test]
    fn bake_recipe_preserves_custom_content() {
        let mut custom_root = HtmlRoot::from(Homemade).lang("fr");
        custom_root.body(HtmlBody::new().content("custom hello"));

        let doc: HtmlDocument = HtmlDocument::from(Homemade)
            .content(custom_root)
            .bake_recipe();

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
    fn recipe_boilerplate() {
        #[derive(Default, Debug, Clone)]
        struct Counter;

        impl ButtonRecipe for Counter {
            recipe_boilerplate!(ButtonRecipe);

            fn content_recipe() -> Self::Content {
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
    fn recipe_boilerplate_custom_content() {
        #[derive(Default, Debug, Clone, Template)]
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

        impl From<TagList> for Bake {
            fn from(list: TagList) -> Self {
                Self::new(&list)
            }
        }

        #[derive(Default, Debug, Clone)]
        struct Tags;

        impl DivRecipe for Tags {
            recipe_boilerplate!(DivRecipe, TagList);

            fn content_recipe() -> Self::Content {
                Self::Content {
                    tags: vec!["foo".into(), "bar".into()],
                }
            }
        }

        let foo_bar = HtmlDiv::from(Tags);
        assert_eq!(foo_bar.bake(), "<div><b>foo</b><b>bar</b></div>");

        let foo_bar_content: TagList = foo_bar.content;
        assert_eq!(foo_bar_content.render().unwrap(), "<b>foo</b><b>bar</b>");

        let baz = HtmlDiv::from(Tags).content(TagList::new("baz"));
        assert_eq!(baz.bake(), "<div><b>baz</b></div>");

        let baz_content: TagList = baz.content;
        assert_eq!(baz_content.render().unwrap(), "<b>baz</b>");

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
}
