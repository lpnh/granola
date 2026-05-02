use granola::{prelude::*, recipes::*, templates::*};

#[derive(Default, Debug, Clone)]
struct FooRecipe;

impl HtmlTag for FooRecipe {
    type Content = HtmlRootContent<Homemade, ()>;

    fn content_recipe(content: &mut Self::Content) {
        let paragraph: HtmlP = HtmlP::new("Hello, world!");

        if let Some(body) = content.body.take() {
            let old_content = body.content;
            let new_content = bake_block![paragraph, old_content];

            let body: HtmlBody = HtmlBody::new(new_content);

            content.body = Some(body);
        } else {
            let body: HtmlBody = HtmlBody::new(bake_newline!(paragraph));

            content.body = Some(body);
        }
    }

    fn decoration_recipe<R: HtmlTag>(element: HtmlRoot<R>) -> HtmlRoot<R> {
        element.class("dark").id("foo-html")
    }
}

impl PTag for FooRecipe {
    type Content = String;

    fn content_recipe(content: &mut Self::Content) {
        *content = bake_block![content, "Content from FooRecipe"];
    }

    fn decoration_recipe<R: PTag>(element: HtmlP<R>) -> HtmlP<R> {
        element.class("text-lg").id("foo-p")
    }
}

impl ButtonTag for FooRecipe {
    fn decoration_recipe<R: ButtonTag>(element: HtmlButton<R>) -> HtmlButton<R> {
        element.class("rounded-full").id("foo-button")
    }
}

#[derive(Default, Debug, Clone)]
struct BarRecipe;

impl HtmlTag for BarRecipe {
    type Content = HtmlRootContent<Homemade, ()>;

    fn decoration_recipe<R: HtmlTag>(element: HtmlRoot<R>) -> HtmlRoot<R> {
        element.add_data("foo", "bar").id("bar-html")
    }
}

impl PTag for BarRecipe {
    type Content = String;

    fn decoration_recipe<R: PTag>(element: HtmlP<R>) -> HtmlP<R> {
        element.add_data("foo", "bar").id("bar-p")
    }
}

impl ButtonTag for BarRecipe {
    fn decoration_recipe<R: ButtonTag>(element: HtmlButton<R>) -> HtmlButton<R> {
        element.add_data("foo", "bar").id("bar-button")
    }
}

#[derive(Default, Debug, Clone)]
struct OneLastRecipe;

impl HtmlTag for OneLastRecipe {
    type Content = HtmlRootContent<Homemade, ()>;

    fn decoration_recipe<R: HtmlTag>(element: HtmlRoot<R>) -> HtmlRoot<R> {
        element.add_data("recipe", "last")
    }
}

impl PTag for OneLastRecipe {
    type Content = String;

    fn content_recipe(content: &mut Self::Content) {
        *content = bake_newline!("Content from OneLastRecipe");
    }

    fn decoration_recipe<R: PTag>(element: HtmlP<R>) -> HtmlP<R> {
        element.add_data("recipe", "last")
    }
}

impl ButtonTag for OneLastRecipe {
    fn decoration_recipe<R: ButtonTag>(element: HtmlButton<R>) -> HtmlButton<R> {
        element.add_data("recipe", "last")
    }
}

fn main() {
    type TmplRecipe = rec!(Homemade, FooRecipe, BarRecipe, OneLastRecipe);

    let tmpl: TmplBase<TmplRecipe> = TmplBase::from_recipe();

    println!("{tmpl}");
}

#[cfg(test)]
mod recipe_tests {
    use super::*;

    #[test]
    fn no_recipe() {
        let button: HtmlButton = HtmlButton::new("Ok");

        let p: HtmlP = HtmlP::new("Oh, hi!");

        let body: HtmlBody = HtmlBody::new(bake_newline!(p));

        let html_root: HtmlRoot = HtmlRoot::new(body);

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
        let button: HtmlButton<FooRecipe> = HtmlButton::empty();

        let p: HtmlP<FooRecipe> = HtmlP::empty();

        let html_root: HtmlRoot<FooRecipe> = HtmlRoot::empty();

        assert_eq!(button.bake(), "<button></button>");
        assert_eq!(p.bake(), "<p></p>");
        assert_eq!(html_root.bake(), "<html></html>");
    }

    #[test]
    fn from_recipe() {
        let button: HtmlButton<FooRecipe> = HtmlButton::from_recipe();

        let p: HtmlP<FooRecipe> = HtmlP::from_recipe();

        let html_root: HtmlRoot<Homemade> = HtmlRoot::from_recipe();

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
        let button: HtmlButton<FooRecipe> = HtmlButton::new("Dismiss");

        let p: HtmlP<FooRecipe> = HtmlP::new("Oh, hi!");

        let body: HtmlBody = HtmlBody::new(bake_newline!(p));

        let html_root: HtmlRoot<Homemade> = HtmlRoot::new(body);

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
    fn composition_from_recipe() {
        let button: HtmlButton<(FooRecipe, BarRecipe)> = HtmlButton::from_recipe();
        let p: HtmlP<(FooRecipe, BarRecipe)> = HtmlP::from_recipe();
        let html_root: HtmlRoot<(Homemade, FooRecipe)> = HtmlRoot::from_recipe();

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
    fn composition_from_recipe_multiple() {
        type LastFooBarRecipe = rec!(FooRecipe, BarRecipe, OneLastRecipe);

        let button: HtmlButton<(Reset, LastFooBarRecipe)> = HtmlButton::from_recipe();

        let p: HtmlP<LastFooBarRecipe> = HtmlP::from_recipe();

        let html_root: HtmlRoot<(Homemade, LastFooBarRecipe)> = HtmlRoot::from_recipe();

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
        let button: HtmlButton<(FooRecipe, BarRecipe)> = HtmlButton::new("Ok");
        let p: HtmlP<(FooRecipe, BarRecipe)> = HtmlP::new("Oh, hi!");

        let body: HtmlBody = HtmlBody::new(bake_newline!(p));
        let html_root: HtmlRoot<(Homemade, FooRecipe)> = HtmlRoot::new(body);

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
        type LastFooBarRecipe = rec!(FooRecipe, BarRecipe, OneLastRecipe);

        let button: HtmlButton<(Reset, LastFooBarRecipe)> = HtmlButton::new("Dismiss");

        let p: HtmlP<LastFooBarRecipe> = HtmlP::new("Oh, hi!");

        let body: HtmlBody = HtmlBody::new(bake_newline!(p));
        let html_root: HtmlRoot<(Homemade, LastFooBarRecipe)> = HtmlRoot::new(body);

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
        type TmplRecipe = rec!(Homemade, FooRecipe, BarRecipe, OneLastRecipe);
        let tmpl: TmplBase<TmplRecipe> = TmplBase::from_recipe();

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
