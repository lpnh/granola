use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<s>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/s)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let s: HtmlS = HtmlS::empty().id("strikethrough");
///
/// assert_eq!(s.bake(),
/// r#"<s id="strikethrough"></s>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let nine: HtmlS = HtmlS::new("nine");
///
/// let solar_system = bake_inline!["Our solar system has ", nine ," eight planets"];
///
/// assert_eq!(solar_system,
/// r#"Our solar system has <s>nine</s> eight planets"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <s{{ attrs }}>{{ content | kirei(2) }}</s>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = STag, content = Cow<'static, str>)]
pub struct HtmlS<M: STag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
}

/// Shorthand for `HtmlS`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let s = s!().id("strikethrough");
///
/// assert_eq!(s.bake(),
/// r#"<s id="strikethrough"></s>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let nine = s!("nine");
///
/// let solar_system = bake_inline!["Our solar system has ", nine ," eight planets"];
///
/// assert_eq!(solar_system,
/// r#"Our solar system has <s>nine</s> eight planets"#);
/// ```
#[macro_export]
macro_rules! s {
    () => {
        $crate::html::HtmlS::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlS::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlS::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlS::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlS::<()>::new($crate::bake_inline![$($content),+])
    };
}
