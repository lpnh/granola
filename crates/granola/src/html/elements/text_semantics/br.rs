use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait BrTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: none, presentation
    const ROLE: Option<&'static str> = None;
}

impl BrTag for () {}

/// The HTML `<br />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/br)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let br: HtmlBr = HtmlBr::new().id("line_break");
///
/// assert_eq!(br.bake(),
/// r#"<br id="line_break" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let br: HtmlBr = HtmlBr::new();
///
/// let roses = bake_inline!["Roses are red,", br];
/// let violets = "Violets are blue.";
///
/// let poem: HtmlP = HtmlP::new(bake_block![roses, violets]);
///
/// assert_eq!(poem.bake(),
/// r#"<p>
///   Roses are red,<br />
///   Violets are blue.
/// </p>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <br
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlBr<M: BrTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: BrTag> HtmlBr<M> {
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

/// Shorthand for `HtmlBr<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let br = br!().id("line_break");
///
/// assert_eq!(br.bake(),
/// r#"<br id="line_break" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let roses = bake_inline!["Roses are red,", br!()];
/// let violets = "Violets are blue.";
///
/// let poem = p!(roses, violets);
///
/// assert_eq!(poem.bake(),
/// r#"<p>
///   Roses are red,<br />
///   Violets are blue.
/// </p>"#);
/// ```
#[macro_export]
macro_rules! br {
    () => {
        $crate::html::HtmlBr::<()>::new()
    };
}
