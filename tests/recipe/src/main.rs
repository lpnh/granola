use granola::{homemade::*, prelude::*, template::*};

#[derive(Default, Debug, Clone)]
struct FooRecipe;

impl HtmlRecipe for FooRecipe {
    type Content = HomemadeRootContent;

    fn content_recipe(content: &mut Self::Content) {
        let paragraph = HtmlP::new().content("Hello, world!");

        if let Some(body) = content.body.take() {
            let old_content = body.content;
            let new_content = bake_block![paragraph, old_content];

            let body = HtmlBody::new().content(new_content);

            content.body = Some(body);
        } else {
            let body = HtmlBody::new().content(bake_newline!(paragraph));

            content.body = Some(body);
        }
    }

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("dark").id("foo-html");
    }
}

impl PRecipe for FooRecipe {
    type Content = String;

    fn content_recipe(content: &mut Self::Content) {
        *content = bake_block![content, "Content from FooRecipe"];
    }

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("text-lg").id("foo-p");
    }
}

impl ButtonRecipe for FooRecipe {
    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("rounded-full").id("foo-button");
    }
}

#[derive(Default, Debug, Clone)]
struct BarRecipe;

impl HtmlRecipe for BarRecipe {
    type Content = HomemadeRootContent;

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.id("bar-html");
    }

    fn custom_data_attrs_recipe(custom_data_attrs: &mut CustomDataAttrs) {
        custom_data_attrs.custom_data("foo", "bar");
    }
}

impl PRecipe for BarRecipe {
    type Content = String;

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.id("bar-p");
    }

    fn custom_data_attrs_recipe(custom_data_attrs: &mut CustomDataAttrs) {
        custom_data_attrs.custom_data("foo", "bar");
    }
}

impl ButtonRecipe for BarRecipe {
    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.id("bar-button");
    }

    fn custom_data_attrs_recipe(custom_data_attrs: &mut CustomDataAttrs) {
        custom_data_attrs.custom_data("foo", "bar");
    }
}

#[derive(Default, Debug, Clone)]
struct OneLastRecipe;

impl HtmlRecipe for OneLastRecipe {
    type Content = HomemadeRootContent;

    fn custom_data_attrs_recipe(custom_data_attrs: &mut CustomDataAttrs) {
        custom_data_attrs.custom_data("recipe", "last");
    }
}

impl PRecipe for OneLastRecipe {
    type Content = String;

    fn content_recipe(content: &mut Self::Content) {
        *content = bake_newline!("Content from OneLastRecipe");
    }

    fn custom_data_attrs_recipe(custom_data_attrs: &mut CustomDataAttrs) {
        custom_data_attrs.custom_data("recipe", "last");
    }
}

impl ButtonRecipe for OneLastRecipe {
    fn custom_data_attrs_recipe(custom_data_attrs: &mut CustomDataAttrs) {
        custom_data_attrs.custom_data("recipe", "last");
    }
}

fn main() {
    type TmplCookbook = cookbook_type![Homemade, FooRecipe, BarRecipe, OneLastRecipe];

    let tmpl: TmplBase<TmplCookbook> = TmplBase::from_cookbook();

    println!("{tmpl}");
}

#[cfg(test)]
mod recipe_tests {
    use granola::recipes::TypeReset;

    use super::*;

    type FooBarLastCookbook = cookbook_type![FooRecipe, BarRecipe, OneLastRecipe];

    #[test]
    fn no_recipe() {
        let button = HtmlButton::new().content("Ok");

        let p = HtmlP::new().content("Oh, hi!");

        let body = HtmlBody::new().content(bake_newline!(p));

        let html_root = HtmlRoot::new().content(body);

        assert_eq!(button.bake(), "<button>Ok</button>");
        assert_eq!(
            html_root.bake(),
            r#"<html>
  <body>
    <p>Oh, hi!</p>
  </body>
</html>"#
        );
    }

    #[test]
    fn empty() {
        let button = HtmlButton::<()>::from_cookbook();

        let p = HtmlP::<()>::from_cookbook();

        let html_root = HtmlRoot::<()>::from_cookbook();

        assert_eq!(button.bake(), "<button></button>");
        assert_eq!(p.bake(), "<p></p>");
        assert_eq!(html_root.bake(), r#"<html></html>"#);
    }

    #[test]
    fn from_cookbook() {
        let button = HtmlButton::from(FooRecipe);

        let p = HtmlP::from(FooRecipe);

        let html_root = HtmlRoot::from(Homemade);

        assert_eq!(
            button.bake(),
            r#"<button class="rounded-full" id="foo-button"></button>"#
        );
        assert_eq!(
            p.bake(),
            r#"<p class="text-lg" id="foo-p">
  Content from FooRecipe
</p>"#
        );
        assert_eq!(
            html_root.bake(),
            r#"<html>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body></body>
</html>"#
        );
    }

    #[test]
    fn new() {
        let button = HtmlButton::from(FooRecipe).content("Dismiss");

        let p = HtmlP::from(FooRecipe).content("Oh, hi!");

        let body = HtmlBody::new().content(bake_newline!(p));

        let html_root = HtmlRoot::from(Homemade).content(body);

        assert_eq!(
            button.bake(),
            r#"<button class="rounded-full" id="foo-button">Dismiss</button>"#
        );
        assert_eq!(
            html_root.bake(),
            r#"<html>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <p class="text-lg" id="foo-p">
      Oh, hi!
      Content from FooRecipe
    </p>
  </body>
</html>"#
        );
    }

    #[test]
    fn composition_from_cookbook() {
        let button = HtmlButton::from((FooRecipe, BarRecipe));
        let p = HtmlP::from((FooRecipe, BarRecipe));
        let html_root = HtmlRoot::from((Homemade, FooRecipe));

        assert_eq!(
            button.bake(),
            r#"<button class="rounded-full" id="bar-button" data-foo="bar"></button>"#
        );
        assert_eq!(
            p.bake(),
            r#"<p class="text-lg" id="bar-p" data-foo="bar">
  Content from FooRecipe
</p>"#
        );
        assert_eq!(
            html_root.bake(),
            r#"<html class="dark" id="foo-html">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <p>Hello, world!</p>
  </body>
</html>"#
        );
    }

    #[test]
    fn composition_from_cookbook_multiple() {
        let button: HtmlButton<(TypeReset, FooBarLastCookbook)> = HtmlButton::from_cookbook();

        let p: HtmlP<FooBarLastCookbook> = HtmlP::from_cookbook();

        let html_root: HtmlRoot<(Homemade, FooBarLastCookbook)> = HtmlRoot::from_cookbook();

        assert_eq!(
            button.bake(),
            r#"<button class="rounded-full" id="bar-button" type="reset" data-foo="bar" data-recipe="last"></button>"#
        );
        assert_eq!(
            p.bake(),
            r#"<p class="text-lg" id="bar-p" data-foo="bar" data-recipe="last">
  Content from OneLastRecipe
</p>"#
        );
        assert_eq!(
            html_root.bake(),
            r#"<html class="dark" id="bar-html" data-foo="bar" data-recipe="last">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <p>Hello, world!</p>
  </body>
</html>"#
        );
    }

    #[test]
    fn composition_new() {
        let button = HtmlButton::from((FooRecipe, BarRecipe)).content("Ok");
        let p = HtmlP::from((FooRecipe, BarRecipe)).content("Oh, hi!");

        let body = HtmlBody::new().content(bake_newline!(p));
        let html_root = HtmlRoot::from((Homemade, FooRecipe)).content(body);

        assert_eq!(
            button.bake(),
            r#"<button class="rounded-full" id="bar-button" data-foo="bar">Ok</button>"#
        );
        assert_eq!(
            html_root.bake(),
            r#"<html class="dark" id="foo-html">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <p>Hello, world!</p>

    <p class="text-lg" id="bar-p" data-foo="bar">
      Oh, hi!
      Content from FooRecipe
    </p>
  </body>
</html>"#
        );
    }

    #[test]
    fn composition_new_multiple() {
        let button =
            HtmlButton::<(TypeReset, FooBarLastCookbook)>::from_cookbook().content("Dismiss");

        let p = HtmlP::<FooBarLastCookbook>::from_cookbook().content("Oh, hi!");

        let body = HtmlBody::new().content(bake_newline!(p));
        let html_root = HtmlRoot::<(Homemade, FooBarLastCookbook)>::from_cookbook().content(body);

        assert_eq!(
            button.bake(),
            r#"<button class="rounded-full" id="bar-button" type="reset" data-foo="bar" data-recipe="last">Dismiss</button>"#
        );
        assert_eq!(
            html_root.bake(),
            r#"<html class="dark" id="bar-html" data-foo="bar" data-recipe="last">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <p>Hello, world!</p>

    <p class="text-lg" id="bar-p" data-foo="bar" data-recipe="last">
      Content from OneLastRecipe
    </p>
  </body>
</html>"#
        );
    }

    #[test]
    fn template() {
        type TmplCookbook = cookbook_type![Homemade, FooRecipe, BarRecipe, OneLastRecipe];

        let tmpl: TmplBase<TmplCookbook> = TmplBase::from_cookbook();

        assert_eq!(
            tmpl.bake(),
            r#"<!doctype html>
<html class="dark" id="bar-html" data-foo="bar" data-recipe="last">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <p>Hello, world!</p>
  </body>
</html>"#
        );
    }
}
