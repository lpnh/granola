use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait HrTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: presentation or none
    const ROLE: Option<&'static str> = None;
}

impl HrTag for () {}

/// The HTML `<hr>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hr)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let hr: HtmlHr = HtmlHr::new().id("thematic_break");
///
/// assert_eq!(hr.bake(),
/// r#"<hr id="thematic_break" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let p1: HtmlP = HtmlP::new("She blew out the candle. The room went dark.");
/// let p2: HtmlP = HtmlP::new("Morning came with birds and the smell of bread from somewhere below.");
///
/// let hr: HtmlHr = HtmlHr::new();
///
/// let story = bake_block![p1, "", hr, "", p2];
///
/// assert_eq!(story,
/// r#"<p>She blew out the candle. The room went dark.</p>
///
/// <hr />
///
/// <p>Morning came with birds and the smell of bread from somewhere below.</p>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <hr
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlHr<M: HrTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: HrTag> HtmlHr<M> {
    pub fn new() -> Self {
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

/// Shorthand for `HtmlHr<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let hr = hr!().id("thematic_break");
///
/// assert_eq!(hr.bake(),
/// r#"<hr id="thematic_break" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let p1 = p!("She blew out the candle. The room went dark.");
/// let p2 = p!("Morning came with birds and the smell of bread from somewhere below.");
///
/// let story = bake_block![p1, "", hr!(), "", p2];
///
/// assert_eq!(story,
/// r#"<p>She blew out the candle. The room went dark.</p>
///
/// <hr />
///
/// <p>Morning came with birds and the smell of bread from somewhere below.</p>"#);
/// ```
#[macro_export]
macro_rules! hr {
    () => {
        $crate::html::HtmlHr::<()>::new()
    };
}
