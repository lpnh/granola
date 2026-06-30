use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<ruby>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ruby)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let ruby = HtmlRuby::new().id("ruby_annotation");
///
/// assert_eq!(ruby.bake(), r#"<ruby id="ruby_annotation"></ruby>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let opening_rp = HtmlRp::new().content("(");
/// let rt = HtmlRt::new().content("とり");
/// let closing_rp = HtmlRp::new().content(")");
///
/// let tori = bake!["鳥", opening_rp, rt, closing_rp];
///
/// let ruby = HtmlRuby::new().content(tori);
///
/// assert_eq!(
///     ruby.bake(),
///     r#"<ruby>鳥<rp>(</rp><rt>とり</rt><rp>)</rp></ruby>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <ruby
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</ruby>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = RubyRecipe, content = Cow<'static, str>)]
pub struct HtmlRuby<R: RubyRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlRuby`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ruby = ruby!().id("ruby_annotation");
///
/// assert_eq!(ruby.bake(), r#"<ruby id="ruby_annotation"></ruby>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let opening_rp = rp!("(");
/// let rt = rt!("とり");
/// let closing_rp = rp!(")");
///
/// let ruby = ruby!("鳥", opening_rp, rt, closing_rp);
///
/// assert_eq!(
///     ruby.bake(),
///     r#"<ruby>鳥<rp>(</rp><rt>とり</rt><rp>)</rp></ruby>"#
/// );
/// ```
#[macro_export]
macro_rules! ruby {
    () => {
        $crate::html::HtmlRuby::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlRuby::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlRuby::new().content($crate::bake![$first $(, $rest)*])
    };
    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlRuby::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; $content:expr $(,)?) => {
        $crate::html::HtmlRuby::<$r>::from_cookbook().content($content)
    };
    (@cookbook $r:ty ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlRuby::<$r>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
