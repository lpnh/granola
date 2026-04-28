use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait ArticleTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: application, document, feed, main, none, presentation, region
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl ArticleTag for () {}

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
/// assert_eq!(article.bake(),
/// r#"<article id="article_contents"></article>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let h2: HtmlH2 = HtmlH2::new("New Café");
///
/// let content = bake_block![
/// "Oats &amp; Ends opened last week on Oak Street,
/// at the corner of Elm Avenue, bringing new aromas to the block.",
/// "Its cozy atmosphere draws in passersby looking to treat themselves to
/// a cup or two of good, hot black coffee."
/// ];
///
/// let p: HtmlP = HtmlP::new(content);
///
/// let article: HtmlArticle = HtmlArticle::new(bake_block![h2, p]);
///
/// assert_eq!(article.bake(),
/// r#"<article>
///   <h2>New Café</h2>
///   <p>
///     Oats &amp; Ends opened last week on Oak Street,
///     at the corner of Elm Avenue, bringing new aromas to the block.
///     Its cozy atmosphere draws in passersby looking to treat themselves to
///     a cup or two of good, hot black coffee.
///   </p>
/// </article>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <article
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</article>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlArticle<M: ArticleTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ArticleTag> HtmlArticle<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }
}

/// Shorthand for `HtmlArticle<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let article = article!().id("article_contents");
///
/// assert_eq!(article.bake(),
/// r#"<article id="article_contents"></article>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let heading = h2!("New Café");
///
/// let paragraph = p![
/// "Oats &amp; Ends opened last week on Oak Street,
/// at the corner of Elm Avenue, bringing new aromas to the block.",
/// "Its cozy atmosphere draws in passersby looking to treat themselves to
/// a cup or two of good, hot black coffee."
/// ];
///
/// let article = article!(heading, paragraph);
///
/// assert_eq!(article.bake(),
/// r#"<article>
///   <h2>New Café</h2>
///   <p>
///     Oats &amp; Ends opened last week on Oak Street,
///     at the corner of Elm Avenue, bringing new aromas to the block.
///     Its cozy atmosphere draws in passersby looking to treat themselves to
///     a cup or two of good, hot black coffee.
///   </p>
/// </article>"#);
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
