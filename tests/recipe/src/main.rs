fn main() {}

#[cfg(test)]
mod tests_recipe {
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
}
