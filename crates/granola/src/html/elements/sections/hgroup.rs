use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<hgroup>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hgroup)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let hgroup = HtmlHgroup::new().id("heading_group");
///
/// assert_eq!(hgroup.bake(), r#"<hgroup id="heading_group"></hgroup>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let heading = HtmlH2::new().content("Capítulo VIII.");
///
/// let subtitle = "Del buen suceso que el valeroso don Quijote tuvo en la espantable y
/// jamás imaginada aventura de los molinos de viento, con otros sucesos
/// dignos de felice recordación";
///
/// let paragraph = HtmlP::new().content(subtitle);
///
/// let hgroup = HtmlHgroup::new().fold_in(heading).fold_in(paragraph);
///
/// assert_eq!(
///     hgroup.bake_pretty(),
///     r#"<hgroup>
///   <h2>Capítulo VIII.</h2>
///   <p>
///     Del buen suceso que el valeroso don Quijote tuvo en la espantable y jamás
///     imaginada aventura de los molinos de viento, con otros sucesos dignos de
///     felice recordación
///   </p>
/// </hgroup>
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <hgroup
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</hgroup>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = HgroupRecipe, content = Cow<'static, str>)]
pub struct HtmlHgroup<R: HgroupRecipe = ()> {
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

/// Shorthand for `HtmlHgroup`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let hgroup = hgroup!().id("heading_group");
///
/// assert_eq!(hgroup.bake(), r#"<hgroup id="heading_group"></hgroup>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let heading = h2!("Capítulo VIII.");
///
/// let subtitle = "Del buen suceso que el valeroso don Quijote tuvo en la espantable y
/// jamás imaginada aventura de los molinos de viento, con otros sucesos
/// dignos de felice recordación";
///
/// let paragraph = p!(subtitle);
///
/// let hgroup = hgroup!(heading, paragraph);
///
/// assert_eq!(
///     hgroup.bake_pretty(),
///     r#"<hgroup>
///   <h2>Capítulo VIII.</h2>
///   <p>
///     Del buen suceso que el valeroso don Quijote tuvo en la espantable y jamás
///     imaginada aventura de los molinos de viento, con otros sucesos dignos de
///     felice recordación
///   </p>
/// </hgroup>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! hgroup {
    () => {
        $crate::html::HtmlHgroup::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlHgroup::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlHgroup::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlHgroup::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; $content:expr $(,)?) => {
        $crate::html::HtmlHgroup::<$r>::from_cookbook().content($content)
    };
    (@cookbook $r:ty ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlHgroup::<$r>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
