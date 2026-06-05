use askama::Template;
use std::borrow::Cow;

use crate::prelude::*;

/// A collection of [`CssDeclaration`].
///
/// The properties-list of [`CssRule`].
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration = CssDeclaration::new("color", "rebeccapurple");
///
/// let css_declarations_block = CssDeclarationsBlock::new().push(css_declaration);
///
/// assert_eq!(css_declarations_block.bake(), "color: rebeccapurple;");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declarations_block = CssDeclarationsBlock::new()
///     .push(("color", "violet"))
///     .push(("font-weight", "lighter"));
///
/// assert_eq!(
///     css_declarations_block.bake(),
///     "color: violet;
/// font-weight: lighter;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {%- for d in declarations -%}
/// {{- d -}}
/// {%- if !loop.last %}
/// {% endif -%}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssDeclarationsBlock {
    pub declarations: Vec<CssDeclaration>,
}

impl CssDeclarationsBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, css_declaration: impl Into<CssDeclaration>) -> Self {
        self.declarations.push(css_declaration.into());
        self
    }

    pub fn push_mut(&mut self, css_declaration: impl Into<CssDeclaration>) -> &mut Self {
        self.declarations.push(css_declaration.into());
        self
    }
}

impl<D: Into<CssDeclaration>, const N: usize> From<[D; N]> for CssDeclarationsBlock {
    fn from(items: [D; N]) -> Self {
        Self {
            declarations: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl<D: Into<CssDeclaration>> From<Vec<D>> for CssDeclarationsBlock {
    fn from(items: Vec<D>) -> Self {
        Self {
            declarations: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CssDeclaration> for CssDeclarationsBlock {
    fn from(css_declaration: CssDeclaration) -> Self {
        Self {
            declarations: vec![css_declaration],
        }
    }
}

impl<P: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>> From<(P, V)> for CssDeclarationsBlock {
    fn from(decl: (P, V)) -> Self {
        Self {
            declarations: vec![decl.into()],
        }
    }
}

/// A CSS property and a CSS value pair.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Syntax/Introduction#css_declarations)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration = CssDeclaration::new("color", "rebeccapurple");
///
/// assert_eq!(css_declaration.bake(), "color: rebeccapurple;");
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ property }}: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssDeclaration {
    pub property: Cow<'static, str>,
    pub value: Cow<'static, str>,
}

impl CssDeclaration {
    pub fn new(
        property: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }
}

impl<P: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>> From<(P, V)> for CssDeclaration {
    fn from((property, value): (P, V)) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }
}

/// Shorthand for `CssDeclarationsBlock`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_declarations_block = declarations_block!(("color", "rebeccapurple"));
///
/// assert_eq!(css_declarations_block.bake(), "color: rebeccapurple;");
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_declarations_block =
///     declarations_block![("color", "violet"), ("font-weight", "lighter")];
///
/// assert_eq!(
///     css_declarations_block.bake(),
///     "color: violet;
/// font-weight: lighter;"
/// );
/// ```
#[macro_export]
macro_rules! declarations_block {
    () => {
        $crate::css::CssDeclarationsBlock::new()
    };
    ($decl:expr $(,)?) => {
        $crate::css::CssDeclarationsBlock::from($decl)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::css::CssDeclarationsBlock::new().push($first)$(.push($rest))*
    };
}

/// Shorthand for `CssDeclaration`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_declaration = declaration!("color", "rebeccapurple");
///
/// assert_eq!(css_declaration.bake(), "color: rebeccapurple;");
/// ```
#[macro_export]
macro_rules! declaration {
    ($prop:expr, $value:expr $(,)?) => {
        $crate::css::CssDeclaration::new($prop, $value)
    };
}
