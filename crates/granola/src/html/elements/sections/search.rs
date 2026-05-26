use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// assert_eq!(search.bake(), r#"<search id="generic_search"></search>"#);
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
/// assert_eq!(
///     search.bake(),
///     r#"<search>
///   <form action="/search">
///     <label for="query">Search the haystack</label>
///     <input id="query" type="search" name="q" placeholder="needle" />
///     <button>Search</button>
///   </form>
/// </search>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <search
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</search>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SearchRecipe, content = Cow<'static, str>)]
pub struct HtmlSearch<R: SearchRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// form, group, none, presentation, region
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlSearch`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let search = search!().id("generic_search");
///
/// assert_eq!(search.bake(), r#"<search id="generic_search"></search>"#);
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
/// assert_eq!(
///     search.bake(),
///     r#"<search>
///   <form action="/search">
///     <label for="query">Search the haystack</label>
///     <input id="query" type="search" name="q" placeholder="needle" />
///     <button>Search</button>
///   </form>
/// </search>"#
/// );
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
