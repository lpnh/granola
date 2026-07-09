use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<article>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/article)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let article = HtmlArticle::new().id("article_contents");
///
/// assert_eq!(
///     article.bake(),
///     r#"<article id="article_contents"></article>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let h2 = HtmlH2::new().content("New Café");
///
/// let content = "Oats &amp; Ends opened last week on Oak Street, at the corner of Elm Avenue, bringing new aromas to the block. Its cozy atmosphere draws in passersby looking to treat themselves to a cup or two of good, hot black coffee.";
///
/// let p = HtmlP::new().content(content);
///
/// let article = HtmlArticle::new().fold_in(h2).fold_in(p);
///
/// assert_eq!(
///     article.bake_pretty(),
///     r#"<article>
///   <h2>New Café</h2>
///   <p>
///     Oats &amp; Ends opened last week on Oak Street, at the corner of Elm Avenue,
///     bringing new aromas to the block. Its cozy atmosphere draws in passersby
///     looking to treat themselves to a cup or two of good, hot black coffee.
///   </p>
/// </article>
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <article
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</article>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ArticleRecipe, content = Bake)]
pub struct HtmlArticle<R: ArticleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// application, document, feed, main, none, presentation, region
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<R: ArticleRecipe<Content = Bake>> HtmlArticle<R> {
    pub fn fold_in(mut self, content: impl FastWritable) -> Self {
        self.content.fold_in(content);
        self
    }
}

/// Shorthand for `HtmlArticle`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let article = article!().id("article_contents");
///
/// assert_eq!(
///     article.bake(),
///     r#"<article id="article_contents"></article>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let heading = h2!("New Café");
///
/// let paragraph = p!("Oats &amp; Ends opened last week on Oak Street, at the corner of Elm Avenue, bringing new aromas to the block. Its cozy atmosphere draws in passersby looking to treat themselves to a cup or two of good, hot black coffee.");
///
/// let article = article!(heading, paragraph);
///
/// assert_eq!(
///     article.bake_pretty(),
///     r#"<article>
///   <h2>New Café</h2>
///   <p>
///     Oats &amp; Ends opened last week on Oak Street, at the corner of Elm Avenue,
///     bringing new aromas to the block. Its cozy atmosphere draws in passersby
///     looking to treat themselves to a cup or two of good, hot black coffee.
///   </p>
/// </article>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! article {
    () => {
        $crate::html::HtmlArticle::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlArticle::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlArticle::new().content($crate::bake![$first $(, $rest)*])
    };

}
