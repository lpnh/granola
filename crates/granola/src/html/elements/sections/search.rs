use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait SearchTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: form, group, none, presentation, region
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl SearchTag for () {}

/// The HTML `<search>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/search)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let search: HtmlSearch = HtmlSearch::empty().id("generic_search");
///
/// assert_eq!(search.bake(),
/// r#"<search id="generic_search"></search>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let label: HtmlLabel = HtmlLabel::new("Search the haystack").for_id("query");
/// let input: HtmlInput = HtmlInput::empty()
///     .input_type("search")
///     .id("query")
///     .name("q")
///     .placeholder("needle");
/// let button: HtmlButton = HtmlButton::new("Search");
///
/// let form: HtmlForm = HtmlForm::new(bake_block![label, input, button]).action("/search");
///
/// let search: HtmlSearch = HtmlSearch::new(form);
///
/// assert_eq!(search.bake(),
/// r#"<search>
///   <form action="/search">
///     <label for="query">Search the haystack</label>
///     <input id="query" type="search" name="q" placeholder="needle" />
///     <button>Search</button>
///   </form>
/// </search>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <search
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</search>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSearch<M: SearchTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SearchTag> HtmlSearch<M> {
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

/// Shorthand for `HtmlSearch<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let search = search!().id("generic_search");
///
/// assert_eq!(search.bake(),
/// r#"<search id="generic_search"></search>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let label = label!("Search the haystack").for_id("query");
/// let input = input!()
///     .input_type("search")
///     .id("query")
///     .name("q")
///     .placeholder("needle");
/// let button = button!("Search");
///
/// let form = form!(label, input, button).action("/search");
///
/// let search = search!(form);
///
/// assert_eq!(search.bake(),
/// r#"<search>
///   <form action="/search">
///     <label for="query">Search the haystack</label>
///     <input id="query" type="search" name="q" placeholder="needle" />
///     <button>Search</button>
///   </form>
/// </search>"#);
/// ```
#[macro_export]
macro_rules! search {
    () => {
        $crate::html::HtmlSearch::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSearch::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSearch::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSearch::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSearch::<()>::new($crate::bake_inline![$($content),+])
    };
}
