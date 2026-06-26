use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `color-mix()` function.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/color_value/color-mix)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_function_color_mix = CssFnColorMix::new()
///     .interpolation(ColorSpace::Oklab)
///     .color_pct("red", "67%")
///     .color("blue");
///
/// assert_eq!(
///     css_function_color_mix.bake(),
///     "color-mix(in oklab, red 67%, blue)"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// color-mix(
///     {%- if let Some(cs) = color_space %}in {{ cs }}, {% endif -%}
///     {%- for c in colors -%}
///         {{ c }}{% if !loop.last %}, {% endif %}
///     {%- endfor -%}
/// )
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FnColorMixRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFnColorMix<R: FnColorMixRecipe = ()> {
    _recipe: PhantomData<R>,
    pub color_space: Option<Cow<'static, str>>,
    pub colors: Vec<Cow<'static, str>>,
}

impl<R: FnColorMixRecipe> CssFnColorMix<R> {
    pub fn interpolation(mut self, color_space: impl Into<Cow<'static, str>>) -> Self {
        self.color_space = Some(color_space.into());
        self
    }

    pub fn color(mut self, color: impl Into<Cow<'static, str>>) -> Self {
        self.colors.push(color.into());
        self
    }

    pub fn color_pct(
        mut self,
        color: impl Into<Cow<'static, str>>,
        percentage: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.colors
            .push(bake_block![color.into(), percentage.into()].into());
        self
    }
}
