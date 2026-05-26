use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

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
/// let article: HtmlArticle = HtmlArticle::empty().id("article_contents");
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
/// let h2: HtmlH2 = HtmlH2::new("New Café");
///
/// let content = bake_block![
///     "Oats &amp; Ends opened last week on Oak Street,
/// at the corner of Elm Avenue, bringing new aromas to the block.",
///     "Its cozy atmosphere draws in passersby looking to treat themselves to
/// a cup or two of good, hot black coffee."
/// ];
///
/// let p: HtmlP = HtmlP::new(content);
///
/// let article: HtmlArticle = HtmlArticle::new(bake_block![h2, p]);
///
/// assert_eq!(
///     article.bake(),
///     r#"<article>
///   <h2>New Café</h2>
///   <p>
///     Oats &amp; Ends opened last week on Oak Street,
///     at the corner of Elm Avenue, bringing new aromas to the block.
///     Its cozy atmosphere draws in passersby looking to treat themselves to
///     a cup or two of good, hot black coffee.
///   </p>
/// </article>"#
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
/// >{{ content | kirei(2) }}</article>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ArticleRecipe, content = Cow<'static, str>)]
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
/// let paragraph = p![
///     "Oats &amp; Ends opened last week on Oak Street,
/// at the corner of Elm Avenue, bringing new aromas to the block.",
///     "Its cozy atmosphere draws in passersby looking to treat themselves to
/// a cup or two of good, hot black coffee."
/// ];
///
/// let article = article!(heading, paragraph);
///
/// assert_eq!(
///     article.bake(),
///     r#"<article>
///   <h2>New Café</h2>
///   <p>
///     Oats &amp; Ends opened last week on Oak Street,
///     at the corner of Elm Avenue, bringing new aromas to the block.
///     Its cozy atmosphere draws in passersby looking to treat themselves to
///     a cup or two of good, hot black coffee.
///   </p>
/// </article>"#
/// );
/// ```
#[macro_export]
macro_rules! article {
    () => {
        $crate::html::HtmlArticle::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlArticle::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlArticle::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlArticle::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlArticle::<()>::new($crate::bake_inline![$($content),+])
    };
}
